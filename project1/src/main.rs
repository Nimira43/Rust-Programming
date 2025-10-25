use std::io;

/// Crate comment
/// This is used to document a Rust program

fn main() {
    
    //! # Input Program
    //! 
    //! ```rust
    //! fn main()
    //! ```
    //! 
    //! Reads user input and then prints it to the console


    let mut input = String::new();
    println!("Hello there!");
    
    /*
    multi 
    line
    comment
    */
    
    // A single line comment
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("How is {}", input.trim());
        },
        Err(e) => {
            println!("Something went wrong! {}", e);
        }
    }
}

