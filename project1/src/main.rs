use std::io;

/// Crate comment
/// This is used to document a Rust program

fn main() {
    // A simple Rust program

    let mut input = String::new();
    println!("Hello there!");
    
    /*
    multi 
    line
    comment
    */

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("How is {}", input.trim());
        },
        Err(e) => {
            println!("Something went wrong! {}", e);
        }
    }
}

