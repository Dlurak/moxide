#[macro_export]
macro_rules! exit {
    ($code:expr, $fmt:expr $(, $args:expr)*) => {{
        eprintln!($fmt $(, $args)*);
        std::process::exit($code);
    }};
}
