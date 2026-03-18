# Invariants — Lean ↔ Rust Mapping

Each invariant is proven formally in Lean and tested numerically in Rust.

## 1. Decomposition Identity

| | |
|---|---|
| **Meaning** | original = trend + seasonality + noise |
| **Lean theorem** | `decomposition_lengths_consistent` (structural: all components same length) |
| **Rust function** | `tensor_net::reconstruct()` |
| **Rust test** | `tests/invariants.rs::decomposition_identity` |
| **Type** | Numeric with tolerance (ε < 1e-10) |
| **Notes** | Lean proves structural consistency (lengths). Rust verifies pointwise numeric equality within floating-point tolerance. |

## 2. Error Non-Negativity

| | |
|---|---|
| **Meaning** | sq_error(original, approximation) ≥ 0 |
| **Lean theorem** | `error_nonneg` (sum of squares ≥ 0 over Nat) |
| **Rust function** | `tensor_net::error_norm()` |
| **Rust test** | `tests/invariants.rs::error_nonneg` |
| **Type** | Exact (Lean), numeric (Rust) |
| **Notes** | Lean proves over Nat (exact arithmetic). Rust tests over f64. Non-negativity holds for both since x² ≥ 0 is type-independent. |

## 3. Error Zero When Identical

| | |
|---|---|
| **Meaning** | sq_error(x, x) = 0 |
| **Lean theorem** | `self_error_zero` (proven by induction on lists) |
| **Rust function** | `tensor_net::error_norm()` |
| **Rust test** | `tests/invariants.rs::error_zero_when_identical` |
| **Type** | Exact (Lean), numeric with tolerance (Rust) |

## 4. Residual Consistency

| | |
|---|---|
| **Meaning** | residual = original - approximation (definitional) |
| **Lean theorem** | `residual_def` (rfl — holds by definition) |
| **Rust function** | `tensor_net::residual()` |
| **Rust test** | `tests/invariants.rs::residual_consistency` |
| **Type** | Exact (both) |
| **Notes** | Lean proves this is definitionally true (`rfl`). Rust verifies with exact floating-point comparison (no rounding in simple subtraction). |

## 5. Residual Length Preservation

| | |
|---|---|
| **Meaning** | length(residual) = length(original) |
| **Lean theorem** | `residual_preserves_length` |
| **Rust function** | `tensor_net::residual()` (returns same-length Array1) |
| **Rust test** | Implicit in all tests (ndarray enforces matching dimensions) |
| **Type** | Exact (both) |

## 6. Reconstruction Length Preservation

| | |
|---|---|
| **Meaning** | length(trend + seasonality + noise) = length(trend) |
| **Lean theorem** | `reconstruct_length` |
| **Rust function** | `tensor_net::reconstruct()` |
| **Rust test** | `tests/invariants.rs::synthetic_reconstruction_identity` |
| **Type** | Exact (both) |

## 7. Structured Preservation

| | |
|---|---|
| **Meaning** | Pure periodic input → noise component is small |
| **Lean theorem** | N/A (empirical property — not formally provable without numeric model) |
| **Rust function** | Decompose + measure noise energy ratio |
| **Rust test** | `tests/invariants.rs::structured_preservation` |
| **Type** | Empirical only |
| **Notes** | This is a statistical property of the SVD algorithm, not a mathematical identity. Rust tests verify noise_energy/signal_energy < 0.05 for pure sine input. |
