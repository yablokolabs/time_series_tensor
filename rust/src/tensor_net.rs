use ndarray::{Array1, Array2};

/// Decomposition result from tensor network.
pub struct TensorDecomposition {
    pub trend: Array1<f64>,
    pub seasonality: Array1<f64>,
    pub noise: Array1<f64>,
}

/// Decompose a time series using tensor network (Hankel matrix + truncated SVD).
///
/// # Method
///
/// 1. **Tensorization**: Reshape the 1D time series into a 2D Hankel (trajectory)
///    matrix where each row is a sliding window of `window_size` consecutive values.
///    This embeds temporal structure into a matrix whose rank reveals the
///    underlying components.
///
/// 2. **Truncated SVD**: Decompose the Hankel matrix via SVD. The singular values
///    are ordered by energy — the first components capture trend (slow movement),
///    middle components capture seasonality (periodic patterns), and the tail
///    captures noise.
///
/// 3. **Component separation**:
///    - **Trend**: Reconstruct from the top `trend_rank` singular components.
///    - **Seasonality**: Reconstruct from the next `seasonal_rank` components.
///    - **Noise**: The residual after removing trend and seasonality.
///
/// 4. **Diagonal averaging**: Convert each rank-reduced Hankel matrix back to
///    a 1D time series by averaging along anti-diagonals (Singular Spectrum
///    Analysis reconstruction).
///
/// This is equivalent to a Matrix Product State (MPS) decomposition with
/// bond dimension = number of retained singular values — the quantum-inspired
/// connection: we're finding a low-rank tensor network representation.
///
/// # Parameters
///
/// - `series`: Input time series of length N.
/// - `window_size`: Embedding window (typically the period, e.g., 7 for weekly).
/// - `trend_rank`: Number of SVD components for trend (typically 1-2).
/// - `seasonal_rank`: Number of SVD components for seasonality (typically 2-4).
pub fn decompose_tensor(
    series: &Array1<f64>,
    window_size: usize,
    trend_rank: usize,
    seasonal_rank: usize,
) -> TensorDecomposition {
    let n = series.len();
    let k = n - window_size + 1;

    // Step 1: Build Hankel (trajectory) matrix
    let mut hankel = Array2::<f64>::zeros((k, window_size));
    for i in 0..k {
        for j in 0..window_size {
            hankel[[i, j]] = series[i + j];
        }
    }

    // Step 2: SVD via power iteration on A^T * A
    let total_rank = (trend_rank + seasonal_rank).min(window_size).min(k);
    let (u, sigma, v) = truncated_svd(&hankel, total_rank);

    // Step 3: Reconstruct components
    let trend_end = trend_rank.min(total_rank);
    let seasonal_end = (trend_rank + seasonal_rank).min(total_rank);

    let trend_hankel = reconstruct_from_svd(&u, &sigma, &v, 0, trend_end);
    let seasonal_hankel = reconstruct_from_svd(&u, &sigma, &v, trend_end, seasonal_end);

    // Step 4: Diagonal averaging
    let trend = diagonal_average(&trend_hankel, n);
    let seasonality = diagonal_average(&seasonal_hankel, n);
    let noise = series - &trend - &seasonality;

    TensorDecomposition {
        trend,
        seasonality,
        noise,
    }
}

/// Truncated SVD via power iteration on the covariance matrix.
///
/// Computes the top `rank` singular triplets (u_i, σ_i, v_i) of matrix A
/// by iteratively solving for eigenvectors of A^T * A (right singular vectors)
/// then deriving left singular vectors via A * v_i / σ_i.
///
/// Uses deflation: after finding each component, subtract its contribution
/// and repeat for the next.
fn truncated_svd(a: &Array2<f64>, rank: usize) -> (Array2<f64>, Array1<f64>, Array2<f64>) {
    let (m, n_cols) = (a.nrows(), a.ncols());
    let mut u_mat = Array2::<f64>::zeros((m, rank));
    let mut sigma = Array1::<f64>::zeros(rank);
    let mut v_mat = Array2::<f64>::zeros((n_cols, rank));

    let mut residual = a.to_owned();

    for r in 0..rank {
        // Power iteration to find dominant right singular vector of residual
        let (s, v_vec, u_vec) = power_iteration(&residual, 200);

        sigma[r] = s;
        for i in 0..m {
            u_mat[[i, r]] = u_vec[i];
        }
        for j in 0..n_cols {
            v_mat[[j, r]] = v_vec[j];
        }

        // Deflate: subtract rank-1 approximation
        for i in 0..m {
            for j in 0..n_cols {
                residual[[i, j]] -= s * u_vec[i] * v_vec[j];
            }
        }
    }

    (u_mat, sigma, v_mat)
}

/// Power iteration: find the dominant singular triplet (σ, v, u) of matrix A.
///
/// Iterates: v ← A^T * A * v (normalized) until convergence,
/// then u ← A * v / σ.
fn power_iteration(a: &Array2<f64>, max_iter: usize) -> (f64, Vec<f64>, Vec<f64>) {
    let (m, n) = (a.nrows(), a.ncols());

    // Initialize with random-ish vector (deterministic for reproducibility)
    let mut v = vec![0.0; n];
    for j in 0..n {
        v[j] = 1.0 / (1.0 + j as f64);
    }
    normalize(&mut v);

    // A^T * A precomputed for efficiency
    let ata = a.t().dot(a);

    for _ in 0..max_iter {
        // w = A^T * A * v
        let mut w = vec![0.0; n];
        for i in 0..n {
            let mut sum = 0.0;
            for j in 0..n {
                sum += ata[[i, j]] * v[j];
            }
            w[i] = sum;
        }
        normalize(&mut w);

        // Check convergence
        let dot: f64 = v.iter().zip(w.iter()).map(|(a, b)| a * b).sum();
        v = w;
        if dot.abs() > 1.0 - 1e-12 {
            break;
        }
    }

    // Compute u = A * v
    let mut u = vec![0.0; m];
    for i in 0..m {
        let mut sum = 0.0;
        for j in 0..n {
            sum += a[[i, j]] * v[j];
        }
        u[i] = sum;
    }

    // σ = ||A * v||
    let sigma: f64 = u.iter().map(|x| x * x).sum::<f64>().sqrt();
    if sigma > 1e-15 {
        for x in u.iter_mut() {
            *x /= sigma;
        }
    }

    (sigma, v, u)
}

fn normalize(v: &mut [f64]) {
    let norm: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm > 1e-15 {
        for x in v.iter_mut() {
            *x /= norm;
        }
    }
}

/// Reconstruct a Hankel matrix from SVD components in range [from, to).
fn reconstruct_from_svd(
    u: &Array2<f64>,
    sigma: &Array1<f64>,
    v: &Array2<f64>,
    from: usize,
    to: usize,
) -> Array2<f64> {
    let k = u.nrows();
    let l = v.nrows(); // v is stored as (n_cols × rank)
    let mut result = Array2::<f64>::zeros((k, l));

    for i in from..to {
        let s = sigma[i];
        for row in 0..k {
            for col in 0..l {
                result[[row, col]] += s * u[[row, i]] * v[[col, i]];
            }
        }
    }

    result
}

/// Diagonal averaging: convert a Hankel matrix back to a 1D time series.
///
/// For a Hankel matrix of size (K × L) derived from a series of length N = K + L - 1,
/// the reconstruction averages all elements along each anti-diagonal.
fn diagonal_average(hankel: &Array2<f64>, n: usize) -> Array1<f64> {
    let k = hankel.nrows();
    let l = hankel.ncols();
    let mut result = Array1::<f64>::zeros(n);
    let mut counts = Array1::<f64>::zeros(n);

    for i in 0..k {
        for j in 0..l {
            let idx = i + j;
            result[idx] += hankel[[i, j]];
            counts[idx] += 1.0;
        }
    }

    result / counts
}

/// Compute the energy (explained variance) captured by each singular value.
///
/// Returns a vector of percentages summing to ~100.
pub fn singular_value_spectrum(series: &Array1<f64>, window_size: usize) -> Vec<f64> {
    let n = series.len();
    let k = n - window_size + 1;

    let mut hankel = Array2::<f64>::zeros((k, window_size));
    for i in 0..k {
        for j in 0..window_size {
            hankel[[i, j]] = series[i + j];
        }
    }

    let (_, sigma, _) = truncated_svd(&hankel, window_size.min(k));
    let total_energy: f64 = sigma.iter().map(|s| s * s).sum();

    sigma.iter().map(|s| 100.0 * s * s / total_energy).collect()
}

// ============================================================================
// Invariant helpers — used by tests and bridge documentation
// ============================================================================

/// Compute the residual: original - approximation.
pub fn residual(original: &Array1<f64>, approximation: &Array1<f64>) -> Array1<f64> {
    original - approximation
}

/// Compute reconstruction from decomposed components: trend + seasonality + noise.
pub fn reconstruct(decomp: &TensorDecomposition) -> Array1<f64> {
    &decomp.trend + &decomp.seasonality + &decomp.noise
}

/// Compute the L2 error norm between original and approximation.
///
/// error_norm(a, b) = sqrt(sum((a - b)^2) / n)
/// This is always ≥ 0.
pub fn error_norm(original: &Array1<f64>, approximation: &Array1<f64>) -> f64 {
    let diff = original - approximation;
    (diff.mapv(|x| x * x).mean().unwrap_or(0.0)).sqrt()
}

/// Compute the approximation (trend + seasonality) without noise.
pub fn approximation(decomp: &TensorDecomposition) -> Array1<f64> {
    &decomp.trend + &decomp.seasonality
}
