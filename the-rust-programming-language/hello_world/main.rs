fn main() {
  println!("Hello, world!");

  res = std::io::stdin().read_line(&mut String::new()).unwrap();
}



// The first line found without a semi-colon is the return value of the function
// fn() {
//  return line 1;
//  line 2
//  line 3; <- unreachable code
// }
