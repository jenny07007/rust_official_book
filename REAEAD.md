#

## 01 rustc

```rs
fu main () {
  // a rust macro
  println!("Hello, world!");
}
```

- An ahead-of-time compiled language, meaning you can compile a program and dive the executable to someone else, and they can run it even without having Rust installed.

## 02 cargo

```sh
cargo new hello-cargo
cargo new --vcs=git hello-cargo # git is the default
cargo build # create an executable file in the target/debug/hello_cargo directory
cargo run # build and run a profect
cargo check # checks code to make sure it compiles but does not produce an executable
cargo build --release # build a release version of the project
```

```sh
git clone example.org/project
cd project
cargo build
```

- `cargo check` is much faster than `cargo build`, because it skips the step of producting an executable. Many Rustanceans run `cargo check` periodically as they write their program to make sure it compiles. Then they run `cargo build` when they're ready to use the executable.

- We can build a project using `cargo build`
- We can build and run a project in one step using `cargo run`
- We can build a project without producing a binary to check for errors using `cargo check`
- Cargo stores save the result of the build in the `targe/debug` directory
- `cargo build --release` to compile with optimaizations and create an executable in `target/rekease`

## 03 Guessing game

- In Rust, variables are immutable by default

```rs
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

```rs
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
```

- We use a `match` expression to decide what to do next based on which variant of `Ordering` was returned from the call to `cmp` with the values in `guess` and `secret_number`.

```rs
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
  }
```

- `i32`, a 32-bit number. `u32` , an unsigned 32-bit number. `i64`, a 64-bit number. `u64`, an unsigned 64-bit number.

```rs
// Rust cannot compare a string and a number type
// shadow the pervious value of guess with a new one
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```
