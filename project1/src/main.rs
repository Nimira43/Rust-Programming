use std::io;

fn main() {
    let mut input = String::new();
    println!("Hello there!");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("How is {}", input.trim());
        },
        Err(e) => {
            println!("Something went wrong! {}", e);
        }
    }
}

