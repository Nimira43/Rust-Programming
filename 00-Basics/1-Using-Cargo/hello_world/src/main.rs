fn main() {
    println!("Hi there");
}

/* Notes

to run                                      rustc main.rs
to run exe                                  ./main
formatting (like Prettier)                  rustfmt main.rs
format all files                            cargo fmt
compiles all code in directory              cargo build
compiles final release code                 cargo build --release
deletes all files in target                 cargo clean

compiles and runs                           cargo run

Compile: rustc main.rs

Run: cargo run

Build: cargo build

Clean: cargo clean

Test: cargo test

Check: cargo check

Doc: cargo doc

Cargo Commands:
New Project: cargo new project_name

Add Dependency: cargo add dependency_name

Update Dependencies: cargo update

List Dependencies: cargo tree

Remove Dependency: cargo rm dependency_name

Rust Compiler Options:
Enable Linting: rustc -- -D warnings

Optimize: rustc --release

Target Specific Architecture: rustc --target <architecture>
*/
