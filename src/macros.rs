#[macro_export]
macro_rules! scanf {
    ($t:ty) => { $crate::scan::<$t>() };
    ($($t:ty),+) => { ($($crate::scan::<$t>(),)*) };
}

/// Macros for writing to stdout
// #[macro_export]
macro_rules! println {
    ($($arg:tt)*) => { {
        use std::io::Write;
        writeln!($crate::stdout(), $($arg)*).unwrap();
    } }
}
// #[macro_export]
macro_rules! print {
    ($($arg:tt)*) => { {
        use std::io::Write;
        write!($crate::stdout(), $($arg)*).unwrap();
    } }
}
pub use scanf;