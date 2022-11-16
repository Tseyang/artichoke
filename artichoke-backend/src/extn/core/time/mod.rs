//! Time is an abstraction of dates and times.
//!
//! This module implements the [`Time`] class from Ruby Core.
//!
//! In Artichoke, Time is represented as a 64-bit signed integer of seconds
//! since January 1, 1970 UTC and an unsigned 32-bit integer of subsecond
//! nanoseconds. This allows representing roughly 584 billion years.
//!
//! You can use this class in your application by accessing it directly. As a
//! Core class, it is globally available:
//!
//! ```ruby
//! Time.now
//! ```
//!
//! This implementation of `Time` supports the system clock via the
//! [`spinoso-time`] crate.
//!
//! [`Time`]: https://ruby-doc.org/core-3.1.2/Time.html

use crate::convert::HeapAllocatedData;
use crate::extn::prelude::*;

pub(in crate::extn) mod mruby;
pub mod offset;
pub mod subsec;
pub mod args;
pub(super) mod trampoline;

#[doc(inline)]
pub use spinoso_time::tzrs::*;

impl HeapAllocatedData for Time {
    const RUBY_TYPE: &'static str = "Time";
}

impl From<TimeError> for Error {
    fn from(error: TimeError) -> Error {
        ArgumentError::from(format!("{error}")).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "Time";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("time_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
