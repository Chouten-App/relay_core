use crate::alloc::string::ToString;
use crate::types::ChoutenError;

#[macro_export]
macro_rules! fail {
    ($msg:expr) => {
        return Err(crate::relay_core::types::ChoutenError::module($msg));
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err(crate::relay_core::types::ChoutenError::module(format!($fmt, $($arg)*)));
    };
}