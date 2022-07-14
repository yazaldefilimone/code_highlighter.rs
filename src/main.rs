#![allow(dead_code)]

mod highlight_error;

fn main() {
  let code = "functon is_zero (x) {
    if (x == 0) [
      return true;
    ] else {
      return false;
    }
  }";
  println!("Error:");
  println!("{}", highlight_error::highlight_error(38, 64, &code));
  println!("");

  let code = "(Foo x) = 7[0 ]\n";
  println!("Error:");
  println!("{}", highlight_error::highlight_error(16, 17, &code));
  println!("");
}
