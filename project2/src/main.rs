#[allow(unused_variables)]

fn main() {

    // Traits
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 30, 30, 30);
    // Debug
    println!("Array: {:?}", [1, 2, 3]);

    // Variables
    let name = "Bob";
    let age = 53;
    let amount1 = 2147483647;
    // let amount2 = 2147483648;         out of range
    let amount2 = -2147483648;
    // let amount2 = -2147483649;        out of range
    let amount: i64 = 2147483648;
    let amount: i64 = -2147483649;
    let unsigned: u32 = 4294967295;
    // let unsigned: u32 = -1;          out of range
    // let unsigned: u32 = 4294967296;  out of range
}
