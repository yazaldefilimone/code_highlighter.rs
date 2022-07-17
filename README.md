Highlight Error
===============

Rust util that highlights an error.

Example
-------

The code below:

```rust
fn main() {
  let code = "functon is_zero (x) {
    if (x == 0) [
      return true;
    ] else {
      return false;
    }
  }";
  println!("{}", highlight_error::highlight_error(38, 64, &code));
}
```

Will output:

![example](./example.png)
