# Quantum-Inspired Time Series Decomposition — Verified Pipeline

A Lean 4 + Rust proof-guided pipeline for time series decomposition using tensor networks (Singular Spectrum Analysis).

**Rust** performs the decomposition. **Lean 4** proves correctness invariants. **Bridge docs** connect them.

## Why Quantum-Inspired?

Tensor networks (Matrix Product States, Tucker decomposition) originate from quantum physics — they're tools for representing high-dimensional quantum states efficiently. The same mathematics applies to classical signal processing: embed a time series into a higher-dimensional tensor (Hankel matrix), then find a low-rank approximation that separates structure from noise.

This project applies that quantum-inspired technique to decompose time series into trend, seasonality, and noise — then formally verifies the decomposition's core properties.

> **Note:** This is a quantum-*inspired* classical algorithm. There is no quantum speedup. The value is in the mathematical framework, not quantum hardware.

## Why Lean + Rust?

| Layer | Role |
|---|---|
| **Rust** | High-performance implementation — SVD, Hankel embedding, plotting |
| **Lean 4** | Formal proofs of simplified correctness invariants |
| **Bridge** | Documentation mapping proofs to implementation |

Lean proves *structural* correctness (decomposition identity, error non-negativity, length preservation). Rust tests verify *numeric* correctness (floating-point tolerance, empirical quality). Together they provide stronger confidence than either alone.

## Repository Structure

```
time_series_tensor/
├── rust/                   # Executable implementation
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs         # Entry point, plotting, comparison
│   │   ├── tensor_net.rs   # Tensor network decomposition (SSA + SVD)
│   │   ├── classical.rs    # Classical baseline (moving average)
│   │   └── data.rs         # Synthetic + CSV data loading
│   ├── tests/
│   │   └── invariants.rs   # Invariant tests (mapped to Lean theorems)
│   ├── data/               # Stock price data
│   └── plots/              # Generated visualizations
├── lean/                   # Formal proofs
│   ├── lakefile.lean
│   ├── Main.lean
│   └── Tensor/
│       ├── Basics.lean         # Vector type, algebraic operations
│       ├── Norms.lean          # Error norms, non-negativity
│       ├── Decomposition.lean  # Decomposition structure, residual
│       └── Theorems.lean       # Core correctness theorems
├── bridge/                 # Lean ↔ Rust mapping
│   ├── INVARIANTS.md       # Each invariant: Lean theorem + Rust test
│   ├── SPEC_MAPPING.md     # Abstract vs concrete, proven vs tested
│   └── examples/
│       └── small_cases.md  # Hand-worked examples
└── README.md
```

## What is Formally Proven (Lean)

| Theorem | Statement |
|---|---|
| `decomposition_lengths_consistent` | All decomposition components have the same length |
| `error_nonneg` | Sum of squared errors ≥ 0 |
| `self_error_zero` | Error between identical sequences = 0 |
| `residual_def` | Residual = original − approximation (definitional) |
| `residual_preserves_length` | Residual has same length as original |
| `reconstruct_length` | Reconstruction preserves length |
| `listZero_length` | Zero list has expected length |

**No `sorry`** — all proofs are complete.

## What is Implemented (Rust)

- Synthetic time series generation (linear trend + weekly seasonality + noise)
- Real-world stock price decomposition (CSV loading)
- Tensor network decomposition via Hankel matrix + truncated SVD
- SVD implemented from scratch (power iteration + deflation — no LAPACK)
- Classical baseline (centered moving average + period averaging)
- Singular value spectrum analysis
- Comparison metrics (RMSE, correlation)
- Visualization via `plotters` (decomposition plots, trend comparison)

## What is Tested (Not Proven)

- Decomposition identity holds within floating-point tolerance (ε < 1e-10)
- Structured inputs (pure sine) have low noise ratio (< 5%)
- Tensor SSA produces smoother trends than classical moving average
- Increasing window captures longer-period patterns
- Stock price decomposition produces meaningful trends

## Build

### Rust

```bash
cd rust
cargo build --release
cargo run --release     # runs decomposition + generates plots
cargo test              # runs invariant tests
```

### Lean

```bash
cd lean
lake build              # compiles all proofs
```

Requires [elan](https://github.com/leanprover/elan) with Lean 4.28.0.

## Results

### Synthetic (365 days)

| Method | Trend RMSE | Seasonality RMSE |
|---|---|---|
| **Tensor (SSA)** | **0.097** | 0.112 |
| Classical (MA) | 0.136 | **0.043** |

### Stock (AAPL simulated, monthly window)

| Metric | Tensor (SSA) | Classical (MA) |
|---|---|---|
| Trend correlation | **0.9975** | 0.9962 |
| Noise std | **$2.37** | $4.38 |

## Limitations

- **Not full formal verification of floating-point numerics.** Lean proofs are over exact types (Nat, structural properties). The Rust f64 implementation is validated empirically.
- **SVD optimality (Eckart-Young theorem) is not formally proven.** Monotonicity is modeled as an axiom on `ApproxOperator` in Lean; Rust tests verify it empirically.
- **No quantum speedup.** This is classical computation using quantum-inspired mathematics.

## License

Apache-2.0
