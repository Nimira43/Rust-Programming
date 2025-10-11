fn main() {
    let grams_of_flour = "84.57";
    println!("{}", grams_of_flour);
    let grams_of_flour = 84.57; // variable shadowing
    println!("{}", grams_of_flour);
    let grams_of_flour = 84; // variable shadowing
    println!("{}", grams_of_flour);
}
