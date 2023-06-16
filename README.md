# ch32v0-fun-debug

Crate for printing into `ch32v003fun` compatible debuggers. (Minichlink and its
derivatives).

# Example usage
```rs
// You can only initialize one instance of the debugger
let mut dbg = unsafe { ch32v003_debug::Debugger::steal() };

// The debugger acts like a serial output
use embedded_hal::serial::Write;
Ok(_) = dbg.write(b"Hello world from Rust\n");
```
