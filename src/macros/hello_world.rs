#[macro_export]

// Task 0.1 — Hello Macro
//
// Objective:
// Create a declarative macro named `hello_world!` that prints a greeting.
//
// Requirements:
// - Use `macro_rules!` only (no procedural macros).
// - Do NOT use any functions.
// - The macro must compile and run successfully with `cargo run`.
//
// Usage:
//     hello_world!();
//
// Expected Output:
//     Hello, world!


macro_rules! hello_world {
    () => {
        println!("Hello, world!");
    };
}
