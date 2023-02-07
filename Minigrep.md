# Setup

```rust
cargo new minigrep
```

👉 [MiniGrep Program](./minigrep/)

## Accepting command line arguments

```sh
cargo run -- searchstring example-filename.txt
```

Collecting the command line arguments into a vector and printing them

```rust
// std::env::args
// we choose to bring the parent module into scope rather than the function
// by doing this, we can easily use other function from std::env
use std::env;

fn main() {
    // collect => to turn the iterator into a vector containing all the values produced by the iterator
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
```

> **The args function and invalid unicode**
> Note that `std::env::args` will panic if any argument contains invalid Unicode. If your program needs to accept arguments containing invalid Unicode, use `std::env::args_os` instead. That function returns an iterator that produces `OsString` values instead of `String` values. We’ve chosen to use `std::env::args` here for simplicity, because `OsString` values differ per platform and are more complex to work with than `String` values.

We can use the `collect` function to create many kinds of collections, so we explicitly annotate the type of args to specify that **we want a vector of strings**. Although we very rarely need to annotate types in Rust, `collect` is one function you do often need to annotate because Rust isn’t able to infer the kind of collection you want.

```sh
# [src/main.rs:5] args = [
#     "target/debug/minigrep", // the name of our binary
# ]
cargo run

# [src/main.rs:5] args = [
#     "target/debug/minigrep",
#     "needle",
#     "haystack",
# ]
cargo run -- needle haystack
```

### Saving the argument values in variables

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // the program's name takes up args[0]
    // args[1] -> the string we search for
    // args[2] -> file_path
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
```

```sh
cargo run -- test sample.txt
```
