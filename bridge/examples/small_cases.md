# Small Cases — Hand-Worked Examples

These examples bridge the Lean proofs to Rust behavior with concrete numbers.

## Example 1: Pure Linear Trend

**Input:** `[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]`

**Expected behavior:**
- Trend ≈ `[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]` (identity for linear input)
- Seasonality ≈ `[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]`
- Noise ≈ `[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]`

**Invariant check:**
```
trend + seasonality + noise = [0, 1, 2, ..., 9] = original ✓
error(original, trend) ≈ 0 (modulo boundary effects) ✓
```

**Rust test:** `small_case_decomposition`

---

## Example 2: Residual Computation

**Input:**
- original = `[10, 20, 30, 40, 50]`
- approximation = `[9.5, 20.5, 29, 41, 49]`

**Residual:**
```
residual = original - approximation
         = [0.5, -0.5, 1.0, -1.0, 1.0]
```

**Squared error:**
```
sq_error = 0.25 + 0.25 + 1.0 + 1.0 + 1.0 = 3.5
RMSE = sqrt(3.5 / 5) = sqrt(0.7) ≈ 0.8367
```

**Invariant checks:**
- `sq_error ≥ 0` ✓ (Lean: `error_nonneg`)
- `residual = original - approx` ✓ (Lean: `residual_def`, by `rfl`)

**Rust test:** `residual_consistency`

---

## Example 3: Self-Error

**Input:** `[3, 1, 4, 1, 5]`

**Computation:**
```
sq_error([3,1,4,1,5], [3,1,4,1,5])
= (3-3)² + (1-1)² + (4-4)² + (1-1)² + (5-5)²
= 0 + 0 + 0 + 0 + 0
= 0
```

**Invariant:** `sq_error(x, x) = 0` ✓ (Lean: `self_error_zero`, proven by induction)

**Rust test:** `error_zero_when_identical`

---

## Example 4: Pure Sine Wave (Structured Input)

**Input:** `sin(2πt/7)` for t = 0..99 (100 points, period = 7)

**Expected behavior:**
- Trend ≈ 0 (no long-term movement)
- Seasonality ≈ original (pure periodic signal)
- Noise ≈ 0 (no random component)
- `noise_energy / signal_energy < 0.05`

**Why this matters:**
If the input is already perfectly structured (periodic), the decomposition
should capture it entirely in the structured components, leaving minimal noise.
This is the "structured preservation" property.

**Rust test:** `structured_preservation`
**Lean analogue:** `perfect_approx_zero_error` (if approx = original, error = 0)

---

## Example 5: Additive Decomposition Identity

**Input:** trend = `[1, 2, 3]`, seasonality = `[0.1, -0.1, 0.1]`, noise = `[0.01, -0.02, 0.03]`

**Original (by construction):**
```
original = trend + seasonality + noise
         = [1.11, 1.88, 3.13]
```

**Reconstruction check:**
```
trend + seasonality + noise = [1.11, 1.88, 3.13] = original ✓
```

**This is trivially true by construction** — which is exactly the point.
The Lean `Decomposition` structure enforces this by requiring the `identity`
proof at construction time. The Rust test verifies it holds through
floating-point arithmetic.
