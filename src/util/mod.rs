#![allow(unused_macros)]
pub mod arena;

pub enum Completion {
    Unfinished,
    Finished,
}

pub fn bi_range<T: PartialOrd>(a: T, b: T) -> std::ops::Range<T> {
    if a < b { a..b } else { b..a }
}

pub fn is_norm(n: f64) -> bool {
    n <= 1. && n >= 0.
}

/// There must exist a bool flag `RUN_DEBUG_MACRO` in the current scope:
/// ```
/// const RUN_DEBUG_MACRO: bool = true;
/// ```
macro_rules! debug {
    ($($code:tt)*) => {
        if RUN_DEBUG_MACRO {
            _ = $($code)*
            ()
        }
        ()
    };
}
