//! Provides a macro `assert_within!` for tests involving floating point numbers.
//!
//! ```rust
//! assert_within!(+0.001, val, target, "Value was not within additive 0.001: {more} {context}");
//! assert_within!(~0.05, val, target, "Value was not within 5% of target: {additional} {information:?}");
//! ```
//!
//! Highlights include:
//!
//! * Pass arguments by reference or value
//! * Sigils (+, ~) indicate additive or relative error
//! * Traps Nan in any of the arguments
//! * Errors cause both the stringified expressions and their values to be displayed
//! * Arbitrary additional format args
//! * Generic over `num_traits::FloatCore`
//! * no_std compatible

#![no_std]

use core::{
    borrow::Borrow,
    fmt::{self, Display},
};
use num_traits::float::FloatCore;

/// Helper for asserting that an f64 value is within +/- epsilon of another, additively
#[doc(hidden)]
#[allow(clippy::too_many_arguments)]
pub fn assert_within_add_impl<N: Display + FloatCore>(
    file: &'static str,
    line: u32,
    val: impl Borrow<N>,
    val_str: &'static str,
    target: impl Borrow<N>,
    target_str: &'static str,
    eps: impl Borrow<N>,
    context: fmt::Arguments,
) {
    let val = val.borrow();
    let target = target.borrow();
    let eps = eps.borrow();

    if eps.is_nan() {
        panic!("assert_within failed at {file}:{line}:\nepsilon was Nan: {eps}\n{context}");
    }

    if *eps < N::zero() {
        panic!(
            "assert_within failed at {file}:{line}:\nEpsilon cannot be negative when used with assert_within! macro: {eps}\n{context}"
        )
    }

    if val.is_nan() {
        panic!("assert_within failed at {file}:{line}:\n`{val_str}` was Nan: {val}\n{context}");
    }

    if target.is_nan() {
        panic!(
            "assert_within failed at {file}:{line}:\n`{target_str}` was Nan: {target}\n{context}"
        );
    }

    if *val < *target - *eps {
        panic!(
            "assert_within failed at {file}:{line}:\n`{val_str}` was less than `{target_str}` - {eps})\nleft:  {val}\nright: {target}\n{context}"
        );
    }

    if *val > *target + *eps {
        panic!(
            "assert_within failed at {file}:{line}:\n`{val_str}` was greater than `{target_str}` + {eps})\nleft:  {val}\nright: {target}\n{context}"
        );
    }
}

/// Helper for asserting that an f64 value is within +/- epsilon of another, multiplicatively
#[doc(hidden)]
#[allow(clippy::too_many_arguments)]
pub fn assert_within_mul_impl<N: Display + FloatCore>(
    file: &'static str,
    line: u32,
    val: impl Borrow<N>,
    val_str: &'static str,
    target: impl Borrow<N>,
    target_str: &'static str,
    eps: impl Borrow<N>,
    context: fmt::Arguments,
) {
    let val = val.borrow();
    let target = target.borrow();
    let eps = eps.borrow();

    if eps.is_nan() {
        panic!("assert_within failed at {file}:{line}:\nepsilon was Nan: {eps}\n{context}");
    }

    if *eps < N::zero() {
        panic!(
            "assert_within failed at {file}:{line}:\nEpsilon cannot be negative when used with assert_within! macro: {eps}\n{context}"
        )
    }

    if val.is_nan() {
        panic!("assert_within failed at {file}:{line}:\n`{val_str}` was Nan: {val}\n{context}");
    }

    if target.is_nan() {
        panic!(
            "assert_within failed at {file}:{line}:\n`{target_str}` was Nan: {target}\n{context}"
        );
    }

    let one_minus_eps = N::one() - *eps;
    if *val < one_minus_eps * *target {
        panic!(
            "assert_within failed at {file}:{line}:\n`{val_str}` was less than (1 ± {eps}) * `{target_str}`\nleft:  {val}\nright: {target}\n{context}"
        );
    }

    let one_plus_eps = N::one() + *eps;
    if *val > one_plus_eps * *target {
        panic!(
            "assert_within failed at {file}:{line}:\n`{val_str}` was greater than (1 ± {eps}) * `{target_str}`\nleft:  {val}\nright: {target}\n{context}"
        );
    }
}

#[macro_export]
macro_rules! assert_within {
    (+ $epsilon:expr, $val:expr, $target:expr) => {
        $crate::test_utils::assert_within_add_impl(
            file!(),
            line!(),
            $val,
            stringify!($val),
            $target,
            stringify!($target),
            $epsilon,
            format_args!(""),
        )
    };

    (+ $epsilon:expr, $val:expr, $target:expr, $($fmt_args:tt)*) => {
        $crate::test_utils::assert_within_add_impl(
            file!(),
            line!(),
            $val,
            stringify!($val),
            $target,
            stringify!($target),
            $epsilon,
            format_args!($($fmt_args)*),
        )
    };

    (~ $epsilon:expr, $val:expr, $target:expr) => {
        $crate::test_utils::assert_within_mul_impl(
            file!(),
            line!(),
            $val,
            stringify!($val),
            $target,
            stringify!($target),
            $epsilon,
            format_args!(""),
        )
    };

    (~ $epsilon:expr, $val:expr, $target:expr, $($fmt_args:tt)*) => {
        $crate::test_utils::assert_within_mul_impl(
            file!(),
            line!(),
            $val,
            stringify!($val),
            $target,
            stringify!($target),
            $epsilon,
            format_args!($($fmt_args)*),
        )
    };
}
