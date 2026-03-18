import Tensor.Basics
import Tensor.Norms

/-!
# Tensor.Decomposition — Decomposition properties

Proves core properties of additive decomposition using lists.
These correspond to the invariant tests in `rust/tests/invariants.rs`.
-/

/-- Residual is defined as original minus approximation (pointwise). -/
def computeResidual (original approx : List Float) : List Float :=
  listSub original approx

/-- Reconstruction: given trend, seasonality, and noise, sum them. -/
def reconstruct (trend seasonality noise : List Float) : List Float :=
  listAdd (listAdd trend seasonality) noise

-- ── Theorem: zipWith length preservation ────────────────────────────

theorem zipWith_length_eq {α β γ : Type} (f : α → β → γ)
    (a : List α) (b : List β) (h : a.length = b.length) :
    (List.zipWith f a b).length = a.length := by
  simp [List.length_zipWith, h]

-- ── Theorem: listAdd preserves length ───────────────────────────────

theorem listAdd_length (a b : List Float) (h : a.length = b.length) :
    (listAdd a b).length = a.length := by
  exact zipWith_length_eq _ a b h

-- ── Theorem: listSub preserves length ───────────────────────────────

theorem listSub_length (a b : List Float) (h : a.length = b.length) :
    (listSub a b).length = a.length := by
  exact zipWith_length_eq _ a b h

-- ── Theorem: Residual has the same length as original ───────────────

theorem residual_length (original approx : List Float)
    (h : approx.length = original.length) :
    (computeResidual original approx).length = original.length := by
  exact listSub_length original approx (by omega)
