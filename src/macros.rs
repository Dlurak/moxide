#[macro_export]
macro_rules! exit {
    ($code:expr, $fmt:expr $(, $args:expr)*) => {{
        eprintln!($fmt $(, $args)*);
        std::process::exit($code);
    }};
}

#[macro_export]
macro_rules! apply_if {
    ($condition:expr, $obj:expr, $method:ident $(, $args:expr)*) => {
        if $condition {
            $obj.$method($($args),*)
        } else {
            $obj
        }
    };
}
