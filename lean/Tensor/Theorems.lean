import Tensor.Decomposition

/-!
# Tensor.Theorems — Core correctness theorems

Proves the main invariants corresponding to Rust tests.
See `bridge/INVARIANTS.md` for the mapping.

## Approach

We prove structural and algebraic properties using Lean's native types:
- List operations for length preservation and structural properties
- Nat arithmetic for non-negativity (exact, no floating-point)
- Float operations for computational examples

The Rust layer handles the numeric (f64) realization with tolerance.
-/

-- ══════════════════════════════════════════════════════════════════════
-- Theorem 1: Decomposition Identity (structural)
-- A valid Decomposition maintains length invariants.
-- Rust test: decomposition_identity
-- ══════════════════════════════════════════════════════════════════════

/-- A valid decomposition has all components the same length. -/
theorem decomposition_lengths_consistent (d : Decomposition) :
    d.trend.length = d.original.length ∧
    d.seasonality.length = d.original.length ∧
    d.noise.length = d.original.length :=
  ⟨d.len_trend, d.len_seasonality, d.len_noise⟩

-- ══════════════════════════════════════════════════════════════════════
-- Theorem 2: Error Non-Negativity
-- Sum of squares is always ≥ 0 (proven over Nat for exactness).
-- Rust test: error_nonneg
-- ══════════════════════════════════════════════════════════════════════

/-- Error (sum of squared differences) is non-negative. -/
theorem error_nonneg (xs : List Nat) :
    (xs.map (fun x => x * x)).foldl (· + ·) 0 ≥ 0 :=
  sum_sq_nonneg xs

-- ══════════════════════════════════════════════════════════════════════
-- Theorem 3: Residual Definition
-- residual = original - approximation (by definition).
-- Rust test: residual_consistency
-- ══════════════════════════════════════════════════════════════════════

/-- Residual is definitionally the pointwise difference. -/
theorem residual_def (original approx : List Float) :
    computeResidual original approx = listSub original approx :=
  rfl

-- ══════════════════════════════════════════════════════════════════════
-- Theorem 4: Residual Length Preservation
-- The residual has the same length as the original.
-- Rust test: residual_consistency
-- ══════════════════════════════════════════════════════════════════════

/-- Residual preserves length. -/
theorem residual_preserves_length (original approx : List Float)
    (h : approx.length = original.length) :
    (computeResidual original approx).length = original.length :=
  residual_length original approx h

-- ══════════════════════════════════════════════════════════════════════
-- Theorem 5: Self-Error is Zero
-- The squared error of a list with itself is zero.
-- Rust test: error_zero_when_identical
-- ══════════════════════════════════════════════════════════════════════

/-- Error between identical sequences is zero (over Nat). -/
theorem self_error_zero (xs : List Nat) :
    (List.zipWith (fun a b => (a - b) * (a - b)) xs xs).foldl (· + ·) 0 = 0 :=
  sq_error_self_nat xs

-- ══════════════════════════════════════════════════════════════════════
-- Theorem 6: Reconstruction Length Preservation
-- Reconstruction output has the same length as inputs.
-- Rust test: synthetic_reconstruction_identity
-- ══════════════════════════════════════════════════════════════════════

/-- Reconstruction (add three components) preserves length. -/
theorem reconstruct_length (t s n : List Float)
    (hts : t.length = s.length) (htn : t.length = n.length) :
    (reconstruct t s n).length = t.length := by
  unfold reconstruct
  have h1 := listAdd_length t s hts
  have h2 := listAdd_length (listAdd t s) n (by omega)
  omega

-- ══════════════════════════════════════════════════════════════════════
-- Theorem 7: Zero List Length
-- listZero n has length n.
-- ══════════════════════════════════════════════════════════════════════

theorem listZero_length (n : Nat) : (listZero n).length = n := by
  unfold listZero
  simp [List.length_replicate]
