#[macro_export]
macro_rules! unwrap_or_panic {
    ($val:expr, $msg:literal) => {
        $val.unwrap_or_else(|| panic!($msg))
    };
    ($val:expr, $fmt:literal, $($args:expr)*) => {
        $val.unwrap_or_else(|| panic!($fmt, $($args)*))
    };
}
