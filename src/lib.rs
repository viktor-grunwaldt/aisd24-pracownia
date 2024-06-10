// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// CODE TAKEN FROM crate 'dmoj' BY Martin Muñoz
// REWRITTEN TO NOT USE ANY DEPENDENCIES BY Viktor Grünwaldt
// FUNCTIONALITY:
// wraps around `read` function from glibc using FFI
// to replace std::io::read_line, and println!
// highly unsafe, maybe even blasphemous, but when dealing with
// raw, ascii input like numbers separated by whitespace
// on a single thread/process,
// it yields substantial perfomance increase (~10x for my simple tests)
// surpassing both C and C++
// works only on UNIX-based systems
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[allow(clippy::upper_case_acronyms)]
// this module could be replaced by OnceLock if rustc version >=1.70
#[macro_use]
mod lazy_static {
    use std::sync::Once;

    pub struct Lazy<T: Sync>(pub *const T, pub Once);

    impl<T: Sync> Lazy<T> {
        #[inline(always)]
        pub fn get<F>(&'static mut self, f: F) -> &T
        where
            F: FnOnce() -> T,
        {
            unsafe {
                let r = &mut self.0;
                self.1.call_once(|| {
                    *r = Box::into_raw(Box::new(f()));
                });

                &*self.0
            }
        }
    }

    macro_rules! __lazy_static_create {
        ($NAME:ident, $T:ty) => {
            use std::sync::Once;
            static mut $NAME: $crate::lazy_static::Lazy<$T> =
                $crate::lazy_static::Lazy(0 as *const $T, Once::new());
        };
    }
    pub use std::ops::Deref as __Deref;

    macro_rules! __lazy_static_internal {
        // optional visibility restrictions are wrapped in `()` to allow for
        // explicitly passing otherwise implicit information about private items
        ($(#[$attr:meta])* ($($vis:tt)*) static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
            __lazy_static_internal!(@MAKE TY, $(#[$attr])*, ($($vis)*), $N);
            __lazy_static_internal!(@TAIL, $N : $T = $e);
            lazy_static!($($t)*);
        };
        (@TAIL, $N:ident : $T:ty = $e:expr) => {
            impl $crate::lazy_static::__Deref for $N {
                type Target = $T;
                fn deref(&self) -> &$T {
                    #[inline(always)]
                    fn __static_ref_initialize() -> $T { $e }

                    #[inline(always)]
                    fn __stability() -> &'static $T {
                        __lazy_static_create!(LAZY, $T);
                        unsafe {LAZY.get(__static_ref_initialize)}
                    }
                    __stability()
                }
            }
            impl $crate::lazy_static::LazyStatic for $N {
                fn initialize(lazy: &Self) {
                    let _ = &**lazy;
                }
            }
        };
        // `vis` is wrapped in `()` to prevent parsing ambiguity
        (@MAKE TY, $(#[$attr:meta])*, ($($vis:tt)*), $N:ident) => {
            #[allow(missing_copy_implementations)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            $(#[$attr])*
            $($vis)* struct $N {__private_field: ()}
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            $($vis)* static $N: $N = $N {__private_field: ()};
        };
        () => ()
    }

    #[macro_export]
    macro_rules! lazy_static {
        ($(#[$attr:meta])* static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
            // use `()` to explicitly forward the information about private items
            __lazy_static_internal!($(#[$attr])* () static ref $N : $T = $e; $($t)*);
        };
        ($(#[$attr:meta])* pub static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
            __lazy_static_internal!($(#[$attr])* (pub) static ref $N : $T = $e; $($t)*);
        };
        ($(#[$attr:meta])* pub ($($vis:tt)+) static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
            __lazy_static_internal!($(#[$attr])* (pub ($($vis)+)) static ref $N : $T = $e; $($t)*);
        };
        () => ()
    }

    /// Support trait for enabling a few common operation on lazy static values.
    ///
    /// This is implemented by each defined lazy static, and
    /// used by the free functions in this crate.
    #[allow(dead_code)]
    pub trait LazyStatic {
        fn initialize(lazy: &Self);
    }
}

mod buf {
    use std::{io::Read, ptr};
    pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;

    pub struct CopyingBufReader<R> {
        inner: R,
        pub buf: Box<[u8]>,
        pub pos: usize,
        pub amt: usize,
    }

    impl<R: Read> CopyingBufReader<R> {
        pub fn new(inner: R) -> CopyingBufReader<R> {
            CopyingBufReader::with_capacity(DEFAULT_BUF_SIZE, inner)
        }

        pub fn with_capacity(capacity: usize, inner: R) -> CopyingBufReader<R> {
            CopyingBufReader {
                inner,
                buf: vec![0; capacity].into_boxed_slice(),
                pos: 0,
                amt: 0,
            }
        }

        pub fn refill(&mut self) {
            let buf_kept = self.amt - self.pos;
            let buf_len = self.buf.len();

            unsafe {
                ptr::copy(
                    self.buf.as_ptr().add(self.pos),
                    self.buf.as_mut_ptr(),
                    buf_kept,
                );
            }

            self.amt = buf_kept + self.inner.read(&mut self.buf[buf_kept..buf_len]).unwrap();
            self.pos = 0;
        }
        #[inline]
        pub fn peek(&mut self) -> Option<u8> {
            if self.pos == self.amt {
                self.refill();
            }

            if self.amt > 0 {
                Some(unsafe { *self.buf.get_unchecked(self.pos) })
            } else {
                None
            }
        }

        pub fn consume(&mut self, amt: usize) {
            assert!(self.pos + amt <= self.amt);
            self.pos += amt;
        }
    }
}

mod io {
    pub use self::stdin::{stdin, Stdin};
    pub use self::stdout::stdout;
    use crate::buf::CopyingBufReader;
    use crate::scan::Scan;

    pub fn scan<T>() -> T
    where
        CopyingBufReader<Stdin>: Scan<T>,
    {
        Scan::<T>::scan(stdin())
    }
    mod stdin {
        const STDIN_FILENO: i32 = 0;
        use std::ffi::c_void;
        use std::io::{Error, Read, Result};

        use crate::buf::CopyingBufReader;
        use crate::lazy_static;
        use crate::sync::NotThreadSafe;

        lazy_static! {
            static ref STDIN: NotThreadSafe<CopyingBufReader<Stdin>> =
                NotThreadSafe::new(CopyingBufReader::new(Stdin::new()));
        }

        pub fn stdin() -> &'static mut CopyingBufReader<Stdin> {
            unsafe { STDIN.get().as_mut().unwrap() }
        }

        pub struct Stdin;

        impl Stdin {
            fn new() -> Stdin {
                Stdin {}
            }
        }

        extern "C" {
            fn read(fd: i32, buf: *mut c_void, count: usize) -> isize;
        }

        impl Read for Stdin {
            fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
                let ret = unsafe { read(STDIN_FILENO, buf.as_mut_ptr() as *mut c_void, buf.len()) };

                if ret == -1 {
                    Err(Error::last_os_error())
                } else {
                    Ok(ret as usize)
                }
            }
        }
    }
    mod stdout {
        use crate::sync::NotThreadSafe;
        use std::io::{self, BufWriter, Stdout, Write};

        extern "C" {
            /// Calls the specified callback function on exit
            fn atexit(callback: extern "C" fn()) -> i32;
        }
        lazy_static! {
            static ref STDOUT: NotThreadSafe<BufWriter<Stdout>> = {
                extern "C" fn flush_stdout_at_exit() {
                    flush();
                }

                unsafe {
                    atexit(flush_stdout_at_exit);
                }

                NotThreadSafe::new(BufWriter::new(io::stdout()))
            };
        }

        pub fn stdout() -> &'static mut BufWriter<Stdout> {
            unsafe { STDOUT.get().as_mut().unwrap() }
        }

        pub fn flush() {
            stdout().flush().unwrap();
        }
    }
}

mod scan {
    use crate::buf::CopyingBufReader;
    use std::io::Read;

    pub trait Scan<T> {
        fn scan(&mut self) -> T;
    }

    macro_rules! impl_scan_signed_integer {
        ($($t:ty)*) => ($(
            impl<R: Read> Scan<$t> for CopyingBufReader<R> {
                fn scan(&mut self) -> $t {
                    let mut neg = false;
                    let mut value: $t = 0;

                    loop {
                        let mut b = self.peek().unwrap();
                        match b {
                            b'-' | b'+' => {
                                neg = b == b'-';
                                self.consume(1);
                                b = self.peek().unwrap();
                                assert!(b.is_ascii_digit());
                            }
                            b'0'..=b'9' => {break;}
                            _ => {self.consume(1);}
                        }

                    }
                    loop {
                        let b = self.peek().unwrap();
                        if !b.is_ascii_digit() {
                            break;
                        }
                        self.consume(1);
                        value = 10 * value + (b - b'0') as $t;
                    }

                    if neg {
                        value = -value;
                    }
                    value
                }
            }
        )*)
    }

    impl_scan_signed_integer!(i8 i16 i32 i64 isize);

    macro_rules! impl_scan_unsigned_integer {
        ($($t:ty)*) => ($(
            impl<R: Read> Scan<$t> for CopyingBufReader<R> {
                fn scan(&mut self) -> $t {
                    let mut value: $t = 0;

                    loop {
                        let b = self.peek().unwrap();
                        if b.is_ascii_digit() {
                            break;
                        }
                        self.consume(1);
                    }

                    loop {
                        let b = self.peek().unwrap();
                        if !b.is_ascii_digit() {
                            break;
                        }
                        self.consume(1);
                        value = 10 * value + (b - b'0') as $t;
                    }

                    value
                }
            }
        )*)
    }
    impl_scan_unsigned_integer!(u8 u16 u32 u64 usize);

    impl<R: Read> Scan<char> for CopyingBufReader<R> {
        fn scan(&mut self) -> char {
            let c = self.peek().unwrap() as char;
            self.consume(1);
            c
        }
    }
}

mod sync {
    use std::cell::UnsafeCell;

    pub struct NotThreadSafe<T> {
        value: UnsafeCell<T>,
    }

    unsafe impl<T> Sync for NotThreadSafe<T> {}

    impl<T> NotThreadSafe<T> {
        pub fn new(value: T) -> NotThreadSafe<T> {
            NotThreadSafe {
                value: UnsafeCell::new(value),
            }
        }

        pub unsafe fn get(&self) -> *mut T {
            self.value.get()
        }
    }
}

pub(crate) use io::{scan, stdout};
pub(crate) mod macros;

pub mod a;
pub mod b;
pub mod c;
pub mod d;
pub mod e;
pub mod f;
pub mod radix;
