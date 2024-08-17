#[macro_export]
macro_rules! exit {
    ($code:expr, $fmt:expr $(, $args:expr)*) => {{
        eprintln!($fmt $(, $args)*);
        std::process::exit($code);
    }};
}

#[macro_export]
macro_rules! conditional_apply {
    ($cmd:expr, $condition:expr, $method:ident $(, $args:expr)*) => {
        if $condition {
            $cmd.$method($($args),*)
        } else {
            $cmd
        }
    };
}
