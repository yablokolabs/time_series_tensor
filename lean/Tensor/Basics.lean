import Lean

/-!
# Tensor.Basics — Abstract vector type and algebraic operations

Lightweight vector abstraction using `List Float` for simplicity.
We prove properties over abstract types where possible, and use
`List Float` for concrete examples.
-/

/-- A decomposition splits a signal into three additive components. -/
structure Decomposition where
  original : List Float
  trend : List Float
  seasonality : List Float
  noise : List Float
  /-- All components have the same length as the original. -/
  len_trend : trend.length = original.length
  len_seasonality : seasonality.length = original.length
  len_noise : noise.length = original.length

/-- Pointwise addition of two lists of the same length. -/
def listAdd (a b : List Float) : List Float :=
  List.zipWith (· + ·) a b

/-- Pointwise subtraction of two lists of the same length. -/
def listSub (a b : List Float) : List Float :=
  List.zipWith (· - ·) a b

/-- The zero list of a given length. -/
def listZero (n : Nat) : List Float :=
  List.replicate n 0.0

/-- Squared values of a list. -/
def listSqElements (a : List Float) : List Float :=
  a.map (fun x => x * x)

/-- Sum of list elements. -/
def listSum (a : List Float) : Float :=
  a.foldl (· + ·) 0.0
