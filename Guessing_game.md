# Guessing Game

- In Rust, variables are immutable by default

```rust
let apple = 5 // immutable
let mut orange = 6 // mutable
```

- `String::new()` => a function that returns a new instance of a string.
- `String` is a string type provided by the standard library that is growable, UTF-8 encoded bit of text. `::new` indicates that `new` is an assocoated function of the `String` type.
- An **associated function** is a function that is impleneted on a type. The `new` function creates a new, empty string.
- `&` indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
- The `Result` types are enumerations, often referred to as `enums`, which can have a fixed set of possibilities known as `variants`.
- `Enums` are often used with `match`, a conditional that makes it convenient to excute different code based on which variant an enum value is when the conditional is evaluated.
- `Result`'s variants are `Ok` and `Err`.

```rust
// create a mutable variable that is currently bond to a new, empty instance of a `String`
let mut guess = String::new();

// same as std::io::stdin
io::stdin()
    .read_line(&mut guess)
    // handling potential falures with the Result Type -- io:Result
    .expect("Failed to read line");
```

### Using a `crate`

- The `Ordering` type is another enum that has two variants, `Less` , `Greater` and `Equal`.
- The `cmp` method compares two values and can be called on anything that can be compared. It takes a reference to the value you want to compare with.
- `crate` is a collection of Rust source code files.The `rand` crate is a binary crate, which contains code intended to be used in orther programs, and can't be excuted on its own.

```toml
rand = "0.8.3"
# add colored dependence
colored = "2.0.0"
```

- We use a `match` expression to decide what to do next based on which variant of `Ordering` was returned from the call to `cmp` with the values in `guess` and `secret_number`.

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
  }
```

- Change texts color

```rust
use colored::*;
match guess.cmp(&secret_number) {
    Ordering::Less => println!("{}", "Too small!".red()),
    Ordering::Greater => println!("{}","Too big!".red()),
    Ordering::Equal => println!("{}","You win!".green()),
  }
```

- `i32`, a 32-bit number. `u32` , an unsigned 32-bit number. `i64`, a 64-bit number. `u64`, an unsigned 64-bit number.

```rust
// Rust cannot compare a string and a number type
// shadow the pervious value of guess with a new one
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```
