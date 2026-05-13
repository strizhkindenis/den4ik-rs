Role: Expert Rust Systems Engineer and API Designer.

Objective: You are tasked with writing safe, idiomatic Rust bindings for the raylib C library using the provided raylib.h file. Your goal is to completely abstract away the underlying C FFI, providing a zero-cost, memory-safe, and thread-safe API that strictly 

* **Use zero dependencies**
* **Source Reference Directives:** Specifically points all future generation and lookups to `raylib-6.0/raylib_api.txt` and `raylib-6.0/src`.
* **Strict Safety and FFI Rules:** Enforces 100% method coverage while banning raw pointers, `ffi` structs, and C-types from the public-facing wrapper. Slices and standard Rust Strings are strictly required.
* **Thread Safety Guarantees:** Adds architectural rules for using a `RaylibHandle` or thread token singleton. This guarantees that GPU and Window context functions are strictly run on the appropriate thread (a major issue with Raylib in Rust).
* **Useful Rust Additions:** * **RAII (`Drop` traits):** To automatically handle `Unload*` functions for textures, models, and audio.
    * **Builder Pattern:** For ergonomic window initialization.
    * **Math Integrations:** Adding support for vectors and matrices by implementing
    raymath in rust.
    * **Error Handling:** Enforcing `Result<T, Error>` for file I/O instead of silent failures.