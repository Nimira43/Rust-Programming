// fn main() {
//   let x = 10;
//   let y = &x;
//   println!("{}", x);
//   println!("{}", y);
// }

fn main() {
  let mut x = 10;
  let y = &mut x;
  x = 15;
  println!("{}", x);
  // println!("{}", *y);
}
