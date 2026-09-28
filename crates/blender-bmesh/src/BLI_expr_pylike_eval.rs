//! Auto-transpiled C/C++ header module: BLI_expr_pylike_eval

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eExprPyLike_EvalStatus {
    EXPR_PYLIKE_SUCCESS = 0,
    EXPR_PYLIKE_DIV_BY_ZERO,
    EXPR_PYLIKE_MATH_ERROR,
    EXPR_PYLIKE_INVALID,
    EXPR_PYLIKE_FATAL_ERROR,
}
