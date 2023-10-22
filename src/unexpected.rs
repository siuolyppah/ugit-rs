#[macro_export]
macro_rules! print_and_exit{

    ($($arg:tt)*) => {{
        println!($($arg)*);
        std::process::exit(-1);
    }};
}
