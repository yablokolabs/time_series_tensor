# Quantum-Inspired Time Series Decomposition with Tensor Networks

A Rust implementation of time series decomposition using tensor networks — specifically, Singular Spectrum Analysis (SSA) via Hankel matrix embedding and truncated SVD with power iteration.

## What it does

Decomposes a time series into three components:
- **Trend**: Smooth, long-term movement
- **Seasonality**: Repeating periodic patterns
- **Noise**: Residuals

## Method

1. **Tensorization**: Reshape the 1D series into a 2D Hankel (trajectory) matrix using a sliding window
2. **Truncated SVD**: Decompose via power iteration — singular values ordered by energy reveal the underlying structure
3. **Component separation**: Top singular components = trend, middle = seasonality, tail = noise
4. **Diagonal averaging**: Reconstruct 1D series from rank-reduced Hankel matrices

This is equivalent to a Matrix Product State (MPS) decomposition with bond dimension = number of retained singular values — the quantum-inspired connection.

## Output

```
Singular value energy spectrum (window=7):
  σ0:  99.87% █████████████████████████████████████████████████
  σ1:   0.06%
  σ2:   0.06%

Tensor Network Decomposition:
  Trend RMSE vs ground truth:        0.0968
  Seasonality RMSE vs ground truth:  0.1123

Classical Decomposition (7-point MA):
  Trend RMSE vs ground truth:        0.1357
  Seasonality RMSE vs ground truth:  0.0434
```

Generates plots in `plots/`:
- `tensor_decomposition.png` — SSA decomposition
- `classical_decomposition.png` — Moving average baseline
- `trend_comparison.png` — Side-by-side trend comparison

## Build & Run

```bash
cargo build --release
cargo run --release
```

No external LAPACK/BLAS dependency — SVD is implemented via power iteration with deflation.

## Project Structure

```
src/
├── main.rs         # Entry point, plotting, comparison
├── tensor_net.rs   # Tensor network decomposition (SSA + truncated SVD)
├── classical.rs    # Classical decomposition (moving average + period averaging)
└── data.rs         # Synthetic time series generation
```

## Why Tensor Networks?

Tensor networks (MPS, Tucker decomposition) are quantum-inspired tools for representing high-dimensional data efficiently. While classical methods (STL, Prophet) are more practical for production time series, this project demonstrates how the same mathematical machinery used in quantum computing (low-rank tensor approximations) applies to classical signal processing.

## License

Apache-2.0
