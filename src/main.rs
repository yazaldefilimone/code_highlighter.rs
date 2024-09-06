#![allow(dead_code)]

mod code_highlighter;

fn main() {
  let code = "functon is_zero (x) {
    if (x == 0) [
      return true;
    ] else {
      return false;
    }
  }";
  println!("Error:");
  println!("{}", code_highlighter::highlight_error_with_context(38, 64, &code, 2));
  println!("");

  println!("Warning:");
  println!("{}", code_highlighter::highlight_warning(38, 64, &code));
  println!("");

  println!("Custom color:");
  println!("{}", code_highlighter::highlight(38, 64, &code, "\x1b[4m\x1b[32m"));
  println!("");

  let code = "(Foo x) = 7[0 ]\n";
  println!("Warning:");
  println!("{}", code_highlighter::highlight_error(16, 17, &code));
  println!("");
}
