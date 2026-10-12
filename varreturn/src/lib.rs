#![no_std]
#![allow(rustdoc::broken_intra_doc_links)]

#![doc = include_str!("../../README.md")]

#[doc(hidden)]
pub mod __private;
mod imp;

/// This macro is used to push a value into the return-array of a function that is marked with the [`varreturn_callee`] attribute macro.
///
/// If you use this macro, you mustn't use the [var_return!](\[expr, ...]) form to avoid missing the pushed values.
///
/// # Panics
/// This macro will panic if the return-array is full. The size of the return-array is determined by [var_return_call] macro.
pub use varreturn_macros::var_push;

/// This macro is used to call a function that is marked with the [`varreturn_callee`] attribute macro.
/// 
/// You can use this macro to get a [`VarReturn`](varreturn::VarReturn).
/// 
/// This macro has two forms:
/// 
/// 1. `var_return!()` - This form is used to return a [`VarReturn`](varreturn::imp::VarReturn) containing the data pushed by [`var_push`].
/// 2. `var_return!([expr, ...])` - This form is used to initialize a new [`VarReturn`](varreturn::imp::VarReturn) with the given expressions.
/// 
/// # Panics
/// When using the `var_return!([expr, ...])` form, this macro will panic if a number of expressions exceeds the buffer size of the return-array or if [`var_push`] is already called in the function.
pub use varreturn_macros::var_return;

/// This attribute macro is used to mark a function as a callee for the [`var_return_call`](varreturn::var_return_call) macro.
/// The attribute macro requires that the function returns a type that implements the [`AssociatedArray`](varreturn::imp::AssociatedArray) trait.
/// 
/// If you want to return values from the function, you can use the [var_return] macro to get [`VarReturn`](varreturn::imp::VarReturn).
/// 
/// If you want to push values into the return-array, you can use the [var_push] macro to push values into the return-array.
pub use varreturn_macros::varreturn_callee;

/// Derives the [`AssociatedArray`] trait for a type. $ty must implement [`AssociatedArray`] trait.
pub use varreturn_macros::VarReturnable;

pub use imp::{AssociatedArray, VarReturn};
