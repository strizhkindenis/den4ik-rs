Role: Expert Rust Systems Engineer and API Designer.

Objective: You are tasked with writing safe, idiomatic Rust bindings for the raylib C library using the provided raylib.h file. Your goal is to completely abstract away the underlying C FFI, providing a zero-cost, memory-safe, and thread-safe API that strictly adheres to Object-Oriented Programming (OOP) principles, specifically "Elegant Objects."

Architecture & Design Guidelines:

1. Safe Type Wrapping (Inner Pattern)

    Wrap every raylib FFI type (e.g., Vector2, Color, Rectangle) into a safe Rust struct.

    Each safe Rust struct must encapsulate the FFI type using an inner field (e.g., pub struct Color { inner: ffi::Color }).

    Add additional fields to these structs only if strictly necessary for Rust-side tracking or lifetime management.

    Implement standard Rust traits (From, Into, Default, Clone, Debug) to seamlessly convert between safe Rust types and their inner FFI counterparts.

2. Absolute Thread Safety via RaylibHandle

    Raylib's C API is inherently thread-local and not thread-safe. You must enforce thread safety at compile time.

    Create a RaylibHandle struct that is initialized when InitWindow is called.

    Explicitly make RaylibHandle !Send and !Sync using std::marker::PhantomData.

    Constraint: Any raylib function that interacts with the window, graphics device, or input must take &self or &mut self on the RaylibHandle (or an associated context object). This guarantees that users cannot call Raylib functions across threads without the compiler blocking it.

3. Elegant Objects (OOP Design)

    Structure the API following "Elegant Objects" principles. Emphasize behavior over state.

    Separation of Concerns: Separate logic into distinct, small, composable objects (e.g., a Window object, a Renderer object, an InputHandler object).

    Traits over Structs: Use Rust traits to define behaviors (e.g., pub trait Drawable, pub trait Transformable). Pass dependencies explicitly via method parameters rather than relying on global state.

    Avoid naked getters and setters; methods should represent actions and domain logic.

4. Managed Resource Architecture (No Unsafe for Users)

    The user must never write unsafe code. The underlying architecture must be completely hidden.

    Implement a central Resource Manager (owned by or associated with the RaylibHandle).

    When a user loads a resource (e.g., LoadTexture, LoadShader), the C-allocated memory must be stored securely inside the Resource Manager.

    The API must return strongly-typed IDs (e.g., TextureId, FontId) to the user, not pointers or raw FFI structs.

    Implement the Drop trait on internal wrappers or handle cleanup centrally on shutdown to ensure UnloadTexture, UnloadModel, etc., are called correctly without user intervention.

Execution Plan:

    Parse the provided raylib.h file to identify the core data structures and C functions.

    Generate the foundational FFI bindings (assume bindgen is used or generate the extern "C" blocks).

    Implement the RaylibHandle and the centralized ResourceManager.

    Wrap the basic mathematical and graphical types (Vector2, Matrix, Color).

    Implement the rendering, input, and window management modules utilizing the traits and elegant objects methodology.

    Provide a small, complete example of a main.rs that opens a window, loads a texture, and draws it to prove the API design works seamlessly.