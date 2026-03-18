import Lake
open Lake DSL

package «tensor-proofs» where
  leanOptions := #[
    ⟨`autoImplicit, false⟩
  ]

@[default_target]
lean_lib «Tensor» where
  srcDir := "."
  roots := #[`Tensor.Basics, `Tensor.Norms, `Tensor.Decomposition, `Tensor.Theorems]
