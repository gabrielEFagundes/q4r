#[macro_export]
macro_rules! die {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        std::process::exit(1);
    }};
}