// Scalar Data Types


fn main() {
    let unsign: u8 = 5; // u16, u32
    let signed: i8 = 5; // i16, i32
    let float: f32 = 5.0;  

    let arch_1: usize = 5;
    let arch_2: isize =  5;

    let char: char = 'a'
    let b: bool = true

    // type aliasing
    type Age = u8;
    let bob_age: Age = 54;
    // type conversion
    let a: i32 = 10;
    let b: f64 = a as f64;

    let fixed_str: &str = "Fixed length string";
    let mut flexible_str: String = String::from("Growing string");

    let mut array_1: [i32; 5] = [4, 5, 6, 7, 8];

    let vec: Vec<i32> = vec![4, 5, 6, 7, 8];

    let my_info: (&str, i32, &str, i32) = ("Salary", 40000, "Age", 40);
    let salary_value: i32 = my_info.1;
    let (salary: &str, salary_value: i32, age: &str, age_value: i32) = my_info;


}

