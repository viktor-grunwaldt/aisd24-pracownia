# aisd24-pracownia

Pracownia z AiSD (2024), Instytut Informatyki, Uniwersytet Wrocławski

## kompilacja

Programy mają być kompilowane i uruchamiane w 64-bitowym środowisku Linux na komputerze PC.
Pamięć cache procesora sprawdzaczki to 3 MB.

Rust, kompilator rustc 1.63.0
rustc –edition=2021 -C opt-level=2 -C target-feature=+crt-static

Wymaganiem jest fakt że kompletny program musi być w jednym pliku, żadne moduły spoza biblioteki standardowej nie będą importowane.
Aby spreparować taki samodzielny plik, każdy z osobnych plików-modułów można przekleić do pliku main.rs wewnątrz bloku `mod (nazwa pliku) { zawartość pliku }`



<details>
  
<summary>Przykład takiego złączonego pliku:</summary>

`lazy_static, buf, io, scan, sync` pochodzą z lib.rs
`macros` pochodzi z macros.rs
`radix` pochodzi z radix.rs

```rust
    #[macro_use]
    mod lazy_static {
        // ...
    }
    
    mod buf {
        // ...
    }
    
    mod io {
        // ...
    }
    
    mod scan {
        // ...
    }
    
    mod sync {
        // ...
    }
    
    mod radix {
        // ...
    }
    pub use io::{scan, stdout};
    
    mod macros {
        #[macro_export]
        macro_rules! scan {
            ($t:ty) => { scan::<$t>() };
            ($($t:ty),+) => { ($(scan::<$t>(),)*) };
        }
    
        /// Macros for writing to stdout
        #[macro_export]
        macro_rules! println {
            ($($arg:tt)*) => { {
                use std::io::Write;
                writeln!($crate::stdout(), $($arg)*).unwrap();
            } }
        }
        #[macro_export]
        macro_rules! print {
            ($($arg:tt)*) => { {
                use std::io::Write;
                write!($crate::stdout(), $($arg)*).unwrap();
            } }
        }
    }
    use radix::sort_by_x;
    
    fn main() {
      // body here
    }
```
</details>
