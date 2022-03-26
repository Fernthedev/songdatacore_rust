use std::i64;

#[must_use = "this returns the result of the operation, \
                        without modifying the original"]
#[inline]
#[track_caller]
#[allow(arithmetic_overflow)]
pub const fn log10(this: i64) -> i64 {
    match checked_log(this, 10) {
        Some(n) => n,
        None => {
            // In debug builds, trigger a panic on None.
            // This should optimize completely out in release builds.
            let _ = i64::MAX + 1;

            0
        }
    }
}

/// Returns the logarithm of the number with respect to an arbitrary base.
///
/// Returns `None` if the number is negative or zero, or if the base is not at least 2.
///
/// This method may not be optimized owing to implementation details;
/// `checked_log2` can produce results more efficiently for base 2, and
/// `checked_log10` can produce results more efficiently for base 10.
///
/// # Examples
///
/// ```
/// #![feature(int_log)]
#[doc = concat!("assert_eq!(5", stringify!($SelfT), ".checked_log(5), Some(1));")]
/// ```
#[must_use = "this returns the result of the operation, \
                        without modifying the original"]
#[inline]
pub const fn checked_log(this: i64, base: i64) -> Option<i64> {
    if this <= 0 || base <= 1 {
        None
    } else {
        let mut n = 0;
        let mut r = this;

        while r >= base {
            r /= base;
            n += 1;
        }
        Some(n)
    }
}
