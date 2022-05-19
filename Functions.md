# Functions

```rs
fn main() {
  another_func(5, 'h');
}

fn another_func(x: i32, y: char) {
  println!("The values are: {} {}", x, y);
}

// Rust is a expression-based language, which means that all statements must have a value.
fn main() {
  let y = {
    let x = 3;
    x + 1
  }
  println!("The value of y is: {}", y); // 4
}
//  functions with return values
fn five() -> i32 {
  5  // !! no semicolon
}
fn main() {
  let x = five();
  println!("The value of x is: {}", x); // 5
}
```
