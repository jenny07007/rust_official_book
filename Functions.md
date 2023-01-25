# Functions

- Either a statement or an expression
  - statement does not return a value
  - expression returns a value

```rust
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

```rust
fn main() {
  let sum: i32 = my_function(x: 11, x: 23);
  println!("The sum is: {}", sum);
}

fn my_function(x:i32, y: i32) -> i32 {
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
  // implicitly return & omit the semicolon cos the 'x+y' is an expression
  // verbose ver = let sum: i32 = x + y;
  x + y
}
```
