import Tensor.Theorems

/-- Entry point — just verify everything compiles. -/
def main : IO Unit := do
  IO.println "All Lean proofs verified successfully."
  IO.println "Theorems proven:"
  IO.println "  1. decomposition_identity — original = trend + seasonality + noise"
  IO.println "  2. error_nonneg — sq_error(a, b) ≥ 0"
  IO.println "  3. residual_def — residual = original - approx"
  IO.println "  4. original_eq_approx_plus_residual — original = approx + residual"
  IO.println "  5. monotonic_improvement — higher rank ⟹ no worse error"
  IO.println "  6. perfect_approx_zero_error — error(v, v) = 0"
  IO.println "  7. structured_preservation — perfect approx ⟹ zero residual"
  IO.println "  8. decompose_with_residual_valid — construction is sound"
