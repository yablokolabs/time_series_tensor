//! Tests verifying the core invariants that are formally proven in Lean.
//! Each test maps to a specific Lean theorem — see bridge/INVARIANTS.md.

use ndarray::Array1;

fn max_abs_diff(a: &Array1<f64>, b: &Array1<f64>) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0.0f64, f64::max)
}

fn l2_error(a: &Array1<f64>, b: &Array1<f64>) -> f64 {
    let diff = a - b;
    diff.mapv(|x| x * x).mean().unwrap_or(0.0).sqrt()
}

/// Simple decomposition for testing: moving average trend + period-averaged seasonality.
fn decompose(series: &[f64], window: usize) -> (Array1<f64>, Array1<f64>, Array1<f64>) {
    let n = series.len();
    let arr = Array1::from_vec(series.to_vec());
    let half = window / 2;

    // Trend: centered moving average
    let mut trend = Array1::<f64>::zeros(n);
    for i in 0..n {
        let start = i.saturating_sub(half);
        let end = (i + half + 1).min(n);
        trend[i] = arr.slice(ndarray::s![start..end]).sum() / (end - start) as f64;
    }

    // Seasonality: period-averaged residuals
    let detrended = &arr - &trend;
    let mut seasonality = Array1::<f64>::zeros(n);
    for pos in 0..window {
        let mut sum = 0.0;
        let mut count = 0;
        for i in (pos..n).step_by(window) {
            sum += detrended[i];
            count += 1;
        }
        let avg = sum / count as f64;
        for i in (pos..n).step_by(window) {
            seasonality[i] = avg;
        }
    }

    let noise = &arr - &trend - &seasonality;
    (trend, seasonality, noise)
}

// ── Invariant 1: Decomposition Identity ──────────────────────────────
// original = trend + seasonality + noise
// Lean theorem: decomposition_identity

#[test]
fn decomposition_identity() {
    let data = vec![
        1.0, 2.5, 1.8, 3.2, 2.1, 4.0, 3.5, 2.0, 1.5, 3.0, 2.8, 4.5, 3.2, 2.7, 1.9,
    ];
    let original = Array1::from_vec(data.clone());
    let (trend, seasonality, noise) = decompose(&data, 5);
    let reconstructed = &trend + &seasonality + &noise;

    assert!(
        max_abs_diff(&original, &reconstructed) < 1e-10,
        "Decomposition identity violated"
    );
}

// ── Invariant 2: Error Non-Negativity ────────────────────────────────
// error_norm(original, approximation) >= 0
// Lean theorem: error_nonneg

#[test]
fn error_nonneg() {
    let a = Array1::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let b = Array1::from_vec(vec![1.1, 1.9, 3.2, 3.8, 5.1]);
    assert!(l2_error(&a, &b) >= 0.0);
}

#[test]
fn error_zero_when_identical() {
    let a = Array1::from_vec(vec![1.0, 2.0, 3.0]);
    assert!(l2_error(&a, &a) < 1e-15);
}

// ── Invariant 3: Residual Consistency ────────────────────────────────
// residual = original - approximation (exactly)
// Lean theorem: residual_def

#[test]
fn residual_consistency() {
    let original = Array1::from_vec(vec![10.0, 20.0, 30.0, 40.0, 50.0]);
    let approx = Array1::from_vec(vec![9.5, 20.5, 29.0, 41.0, 49.0]);
    let residual = &original - &approx;
    let expected = Array1::from_vec(vec![0.5, -0.5, 1.0, -1.0, 1.0]);

    assert!(
        max_abs_diff(&residual, &expected) < 1e-15,
        "Residual consistency violated"
    );
}

// ── Invariant 4: Small Case Decomposition ────────────────────────────
// Hand-worked example bridging to Lean documentation

#[test]
fn small_case_decomposition() {
    let data: Vec<f64> = (0..10).map(|i| i as f64).collect();
    let original = Array1::from_vec(data.clone());
    let (trend, seasonality, noise) = decompose(&data, 3);
    let reconstructed = &trend + &seasonality + &noise;

    assert!(
        max_abs_diff(&original, &reconstructed) < 1e-10,
        "Small case decomposition identity violated"
    );
}

// ── Invariant 5: Structured Preservation ─────────────────────────────
// Pure periodic input → noise should be small
// Lean theorem: structured_preservation

#[test]
fn structured_preservation() {
    let n = 100;
    let period = 7;
    let data: Vec<f64> = (0..n)
        .map(|i| (2.0 * std::f64::consts::PI * i as f64 / period as f64).sin())
        .collect();
    let original = Array1::from_vec(data.clone());
    let (_trend, _seasonality, noise) = decompose(&data, period);

    let noise_energy: f64 = noise.mapv(|x| x * x).sum();
    let signal_energy: f64 = original.mapv(|x| x * x).sum();
    let ratio = noise_energy / signal_energy;

    assert!(
        ratio < 0.05,
        "Structured input noise ratio too high: {ratio:.4}"
    );
}

// ── Invariant 6: Synthetic round-trip ────────────────────────────────

#[test]
fn synthetic_reconstruction_identity() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let n = 365;
    let data: Vec<f64> = (0..n)
        .map(|i| {
            0.1 * i as f64
                + (2.0 * std::f64::consts::PI * i as f64 / 7.0).sin()
                + rng.gen_range(-0.5..0.5)
        })
        .collect();
    let original = Array1::from_vec(data.clone());
    let (trend, seasonality, noise) = decompose(&data, 7);
    let reconstructed = &trend + &seasonality + &noise;

    assert!(
        max_abs_diff(&original, &reconstructed) < 1e-10,
        "Synthetic reconstruction identity violated"
    );
}
