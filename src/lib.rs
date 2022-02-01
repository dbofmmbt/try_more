//! # Expand your possibilities with the Try `?` Operator
//!
//! Have you ever found yourself writing a function which may return early based on some condition?
//!
//! ```rust
//! fn my_function() {
//!     // ...
//!
//!     # let condition_a = false;
//!     if condition_a {
//!         return;
//!     }
//!     
//!     // ...
//!     
//!     # let condition_b = true;
//!     if condition_b {
//!         return;        
//!     }
//!     
//!     // ...
//! }
//! ```
//!
//! It doesn't look Rusty, right? This crate offers an extension trait to be able to convert from
//! a `bool` to a `ControlFlow` and leverage the mighty power of `?` to get rid of those checks:
//!
//! ```rust
//! # use core::ops::ControlFlow;
//! # use try_more::BoolFlow;
//!
//! fn my_function() -> ControlFlow<()> {
//!     // ...
//!     # let condition_a = false;
//!     BoolFlow::r#break(condition_a)?;

//!     // ...
//!     # let condition_b = true;
//!     condition_b.r#break()?;
//!     
//!     // ...
//!     # ControlFlow::Continue(())
//! }
//! ```
//!
//! There's also other methods besides [continue][BoolFlow::continue] and [break][BoolFlow::break] which allows to control the value which is passed to the `Break` variant.
//!

use std::ops::ControlFlow;

use private::Sealed;

mod private {
    pub trait Sealed {}
}

/// Allows to convert from a `bool` to a `ControlFlow` in order to easily use the `?` operator.
pub trait BoolFlow: Sealed {
    /// Returns `ControlFlow::Break` if `self` is true.
    fn r#break(self) -> ControlFlow<()>;

    /// Returns `ControlFlow::Break(T)` if `self` is true.
    fn break_with<T>(self, value: T) -> ControlFlow<T>;

    /// Lazily returns `ControlFlow::Break(T)` if `self` is true.
    fn break_lazy<T>(self, value: impl FnOnce() -> T) -> ControlFlow<T>;

    /// If `self` is `true`, it returns `ControlFlow::Continue`. `ControlFlow::Break(())` otherwise.
    fn r#continue(self) -> ControlFlow<()>;

    /// If `self` is `true`, it returns `ControlFlow::Continue`. `ControlFlow::Break(T)` otherwise.
    fn continue_or<T>(self, value: T) -> ControlFlow<T>;

    /// If `self` is `true`, it returns `ControlFlow::Continue`. Lazily returns `ControlFlow::Break(T)` otherwise.
    fn continue_or_else<T>(self, f: impl FnOnce() -> T) -> ControlFlow<T>;
}

impl Sealed for bool {}

impl BoolFlow for bool {
    fn r#break(self) -> ControlFlow<()> {
        match self {
            true => ControlFlow::Break(()),
            false => ControlFlow::Continue(()),
        }
    }

    fn break_with<T>(self, value: T) -> ControlFlow<T> {
        match self {
            true => ControlFlow::Break(value),
            false => ControlFlow::Continue(()),
        }
    }

    fn break_lazy<T>(self, f: impl FnOnce() -> T) -> ControlFlow<T> {
        match self {
            true => ControlFlow::Break(f()),
            false => ControlFlow::Continue(()),
        }
    }

    fn r#continue(self) -> ControlFlow<()> {
        match self {
            true => ControlFlow::Continue(()),
            false => ControlFlow::Break(()),
        }
    }

    fn continue_or<T>(self, value: T) -> ControlFlow<T> {
        match self {
            true => ControlFlow::Continue(()),
            false => ControlFlow::Break(value),
        }
    }

    fn continue_or_else<T>(self, f: impl FnOnce() -> T) -> ControlFlow<T> {
        match self {
            true => ControlFlow::Continue(()),
            false => ControlFlow::Break(f()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::ControlFlow;

    use crate::BoolFlow;

    #[test]
    fn break_works() {
        fn r#break(condition: bool) -> ControlFlow<()> {
            BoolFlow::r#break(condition)?;
            ControlFlow::Continue(())
        }

        fn break_with(condition: bool) -> ControlFlow<bool> {
            condition.r#break_with(true)?;
            ControlFlow::Break(false)
        }

        fn break_lazy(condition: bool) -> ControlFlow<bool> {
            condition.r#break_lazy(|| true)?;
            ControlFlow::Break(false)
        }

        assert_eq!(r#break(true), ControlFlow::Break(()));
        assert_eq!(break_with(true), ControlFlow::Break(true));
        assert_eq!(break_lazy(true), ControlFlow::Break(true));

        assert_eq!(r#break(false), ControlFlow::Continue(()));
        assert_eq!(break_with(false), ControlFlow::Break(false));
        assert_eq!(break_lazy(false), ControlFlow::Break(false));
    }

    #[test]
    fn continue_works() {
        fn should_continue(condition: bool, continued: &mut bool) -> ControlFlow<()> {
            BoolFlow::r#continue(condition)?;
            *continued = true;
            ControlFlow::Break(())
        }

        fn test_continue_or(condition: bool) -> ControlFlow<bool> {
            condition.continue_or(true)?;
            ControlFlow::Break(false)
        }

        fn test_continue_or_else(condition: bool) -> ControlFlow<bool> {
            condition.continue_or_else(|| true)?;
            ControlFlow::Break(false)
        }

        {
            let mut continued = false;
            should_continue(true, &mut continued);
            assert!(continued);
        }
        assert_eq!(test_continue_or(true), ControlFlow::Break(false));
        assert_eq!(test_continue_or_else(true), ControlFlow::Break(false));

        {
            let mut continued = false;
            should_continue(false, &mut continued);
            assert!(!continued);
        }
        assert_eq!(test_continue_or(false), ControlFlow::Break(true));
        assert_eq!(test_continue_or_else(false), ControlFlow::Break(true));
    }
}
