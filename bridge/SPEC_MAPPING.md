# Specification Mapping — Lean Abstractions vs Rust Implementations

## What is abstract in Lean

| Concept | Lean representation | Why abstract |
|---|---|---|
| **Vector/Signal** | `List Float` or `List Nat` | Avoids ndarray dependency; focuses on algebraic properties |
| **Addition/Subtraction** | `listAdd`, `listSub` (pointwise zipWith) | Matches Rust semantics without linear algebra |
| **Error norm** | Sum of squares over `Nat` | Exact arithmetic — no floating-point rounding |
| **Decomposition** | `Decomposition` structure with length proofs | Structural correctness, not numeric values |
| **Approximation operator** | Not formalized (would need SVD theory) | Too complex for lightweight proofs; tested empirically |

## What is concrete in Rust

| Concept | Rust representation | Why concrete |
|---|---|---|
| **Vector/Signal** | `ndarray::Array1<f64>` | High-performance, cache-friendly arrays |
| **Hankel matrix** | `ndarray::Array2<f64>` | 2D trajectory matrix for SSA embedding |
| **SVD** | Power iteration with deflation | Custom implementation — no LAPACK dependency |
| **Trend/Seasonality** | Rank-separated SVD components + diagonal averaging | Full numeric pipeline |
| **Plotting** | `plotters` crate | Visualization of decomposed components |

## Where models intentionally differ

| Aspect | Lean | Rust | Why |
|---|---|---|---|
| **Arithmetic** | Exact (Nat, or Float without proofs about precision) | f64 with IEEE 754 rounding | Lean proofs are about structure, not floating-point behavior |
| **Error metric** | Sum of squares (no sqrt) | RMSE (with sqrt) | Lean avoids irrational numbers; Rust uses standard RMSE |
| **Decomposition** | Three-component additive split (by definition) | SVD-based with configurable rank | Lean proves the split is valid; Rust chooses how to split |
| **Monotonicity** | Assumed as axiom on `ApproxOperator` | Observed empirically (SVD is optimal by Eckart-Young) | Full SVD optimality proof requires deep linear algebra |

## What is formally proven vs empirically validated

### Formally proven (Lean)
- Decomposition identity (structural: same lengths)
- Error non-negativity (x² ≥ 0)
- Self-error = 0
- Residual = original - approximation (definitional)
- Length preservation through operations

### Empirically validated (Rust tests)
- Decomposition identity (numeric: max pointwise error < 1e-10)
- SVD-based trend extracts meaningful structure
- Structured inputs have low noise ratio (< 5%)
- Tensor network beats classical moving average on trend correlation
- Increasing window size captures longer-period patterns
