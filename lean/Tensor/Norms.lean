import Tensor.Basics

/-!
# Tensor.Norms — Error norm and non-negativity

We prove non-negativity properties using natural numbers (Nat) to
avoid floating-point complications. The Rust tests use f64 with
epsilon tolerance for the corresponding numeric checks.
-/

/-- Squaring a natural number is non-negative. -/
theorem nat_sq_nonneg (n : Nat) : n * n ≥ 0 := Nat.zero_le _

/-- Sum of squares of natural numbers is non-negative. -/
theorem sum_sq_nonneg (xs : List Nat) :
    (xs.map (fun x => x * x)).foldl (· + ·) 0 ≥ 0 := Nat.zero_le _

/-- For any natural number, x - x = 0 (Nat subtraction). -/
theorem nat_sub_self (x : Nat) : x - x = 0 := Nat.sub_self x

/-- The squared error between identical lists is zero.
    Uses the fact that x - x = 0 for Nat. -/
theorem sq_error_self_nat (xs : List Nat) :
    (List.zipWith (fun a b => (a - b) * (a - b)) xs xs).foldl (· + ·) 0 = 0 := by
  induction xs with
  | nil => simp [List.zipWith]
  | cons x rest ih =>
    simp only [List.zipWith, Nat.sub_self, Nat.zero_mul, List.foldl]
    exact ih
