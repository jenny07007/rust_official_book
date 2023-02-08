- [Setup](#setup)
  - [Accepting command line arguments](#accepting-command-line-arguments)
    - [Saving the argument values in variables](#saving-the-argument-values-in-variables)
    - [Reading a file](#reading-a-file)
    - [Refactoring to improve modularity and errir handling](#refactoring-to-improve-modularity-and-errir-handling)
    - [Grouping configuration values](#grouping-configuration-values)
    - [Creating a constructor for `Config`](#creating-a-constructor-for-config)
    - [Fixing the error handling](#fixing-the-error-handling)
    - [Returning a `Result` instead of calling `panic!`](#returning-a-result-instead-of-calling-panic)
    - [Extracing logic from `main`](#extracing-logic-from-main)
    - [Returning errors from the `run` function](#returning-errors-from-the-run-function)
    - [Splitting code into a library crate](#splitting-code-into-a-library-crate)
  - [Developing the library's functionality with TDD(Test-Driven Development)](#developing-the-librarys-functionality-with-tddtest-driven-development)
    - [Writing code to pass the test](#writing-code-to-pass-the-test)
  - [Working with Enviornment Variables](#working-with-enviornment-variables)
  - [Writing error messages to standard error instead of standard output](#writing-error-messages-to-standard-error-instead-of-standard-output)
    - [Checking where errors are written](#checking-where-errors-are-written)
    - [Printing errors to standard error](#printing-errors-to-standard-error)

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

### Reading a file

```sh
cargo run -- the poem.txt
```

```rust
use std::env;
use std::fs;

fn main() {
    // -- snip --
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}")
}
```

### Refactoring to improve modularity and errir handling

This pattern is about separating concerns: `main.rs` handles running the program, and `lib.rs` handles all the logic of the task at hand. Because you can’t test the main function directly, this structure lets you test all of your program’s logic by moving it into functions in `lib.rs`. The code that remains in `main.rs` will be small enough to verify its correctness by reading it.

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);

    // --snip--

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    // return a tuple, but then we immediately break that truple into individual parts again. this's a sign that perhaps we dont hve the right abstraction yet.
    (query, file_path)
}
```

### Grouping configuration values

Another indicator that shows there’s room for improvement is the `config` part of `parse_config`, which implies that the two values we return are related and are both part of one configuration value. We’re not currently conveying this meaning in the structure of the data other than by grouping the two values into a tuple; we’ll instead put the two values into one `struct` and give each of the `struct` fields a meaningful name. Doing so will make it easier for future maintainers of this code to understand how the different values relate to each other and what their purpose is.

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
```

The `args` variable in `main` is the owner of the argument values and is only letting the `parse_config` function borrow them, which means we’d violate Rust’s borrowing rules if `Config` tried to take ownership of the values in `args`.

There are a number of ways we could manage the `String` data; the easiest, though somewhat inefficient, route is to call the `clone` method on the values. This will make a full copy of the data for the `Config` instance to own, which takes more time and memory than storing a reference to the string data. However, cloning the data also makes our code very straightforward because we don’t have to manage the lifetimes of the references; in this circumstance, giving up a little performance to gain simplicity is a worthwhile trade-off.

### Creating a constructor for `Config`

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
// --snip--
}
// --snip--
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config {query, file_path}
    }
}
```

### Fixing the error handling

The line `index out of bounds: the len is 1 but the index is 1` is an error message intended for programmers. It won’t help our end users understand what they should do instead. Let’s fix that now.

```rust
// --snip--
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // --snip--
}
```

### Returning a `Result` instead of calling `panic!`

Return a `Result` value that will contain a `Config` instance and a `&'static str` in the error case. Our error values will always be string literals that have the `'static` lifetime.

```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
```

Returning a `Err` value from `Config::build` allows the `main` function to handle the `Result` value returned from the `build` and exit the process more cleanly in the error case.

`unwrap_or_else`, which is defined on `Result<T, E>` by the standard library. Using `unwrap_or_else` allows us to define some custom, non-`panic!` error handling. If the Result is an `Ok` value, this method’s behavior is similar to `unwrap`: it returns the inner value `Ok` is wrapping. However, if the value is an `Err` value, this method calls the code in the **closure**, which is an anonymous function we define and pass as an argument to `unwrap_or_else`.

```rust
// exit the program without panicking
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
}
```

The code in the closure that will be run in the error case is only two lines: we print the `err` value and then call `process::exit`. The `process::exit` function will stop the program immediately and return the number that was passed as the exit status code.

### Extracing logic from `main`

```rust
fn run(config: Config) {
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}")
}
```

### Returning errors from the `run` function

First, we changed the return type of the run function to `Result<(), Box<dyn Error>>`. This function previously returned the unit type, `()`, and we keep that as the value returned in the `Ok` case.

For the error type, we used the trait object `Box<dyn Error>` (and we’ve brought `std::error::Error` into scope with a use statement at the top). We’ll cover **trait objects** in Chapter 17. For now, just know that `Box<dyn Error>` means the function will return a type that implements the `Error` trait, but we don’t have to specify what particular type the return value will be. This gives us flexibility to return error values that may be of different types in different error cases. **The dyn keyword is short for “dynamic.”**

Second, we’ve removed the call to expect in favor of the `?` operator, as we talked about in Chapter 9. Rather than `panic!` on an error, **`?` will return the error value from the current function for the caller to handle.**

Third, the `run` function now returns an `Ok` value in the success case. We’ve declared the `run` function’s success type as `()` in the signature, which means we need to wrap the unit type value in the `Ok` value. This `Ok(())` syntax might look a bit strange at first, but using `()` like this is the idiomatic way to indicate that **we’re calling `run` for its side effects only; it doesn’t return a value we need**.

```rust
use std::error::Error;

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
```

We use `if let` rather than `unwrap_or_else` to check whether run returns an `Err` value and call `process::exit(1)` if it does. The run function doesn’t return a value that we want to `unwrap` in the same way that `Config::build` returns the `Config` instance. Because `run` returns `()` in the success case, we only care about detecting an error, so we don’t need `unwrap_or_else` to return the unwrapped value, which would only be `()`.

The bodies of the if let and the `unwrap_or_else` functions are the same in both cases: we print the error and exit.

```rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    //
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
```

### Splitting code into a library crate

Let’s move all the code that isn’t the main function from src/main.rs to src/lib.rs:

- The run function definition
- The relevant use statements
- The definition of `Config`
- The `Config::build` function definition

## Developing the library's functionality with TDD(Test-Driven Development)

Add the searching logic to the `minigrep` program using RDD process with the following steps:

1. Write a test that fails and run it to make sure it fails for the reason you expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests continue to pass.
4. Repeat from step 1!

```rust
// when we return a reference from a function, we have to tie the lifetime if that reference to the lifetime of one of the input parameters
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
```

Notice that we need to define an explicit lifetime `'a` in the signature of `search` and use that lifetime with the `contents` argument and the return value. The lifetime parameters specify which argument lifetime is connected to the lifetime of the return value. In this case, we indicate that the returned vector should contain **string slices** that reference slices of the argument `contents` (rather than the argument query).

In other words, we tell Rust that the data returned by the `search` function will live as long as the data passed into the `search` function in the `contents` argument. This is important! The data referenced by a slice needs to be valid for the reference to be valid; if the compiler assumes we’re making **string slices** of `query` rather than `contents`, it will do its safety checking incorrectly.

### Writing code to pass the test

Our program needs to follow these steps:

1. Iterate through each line of the contents.
2. Check whether the line contains our query string.
3. If it does, add it to the list of values we’re returning.
4. If it doesn’t, do nothing.
5. Return the list of results that match.

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}
```

Using the `search` function in the `run` function

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
```

## Working with Enviornment Variables

An option for case-insensitive searching that the user can turn on via an env.

```rust
pub fn search_case_insentitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // query is a String rather than a string slice
    // calling to_lowercase() creates new data rather than referencing existing data
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.line() {
        // we beed to add an ampersand '&'
        // because the contains method is defined to take a string slice
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}


#[test]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insentitive(query, contents)
        )
    }
}
```

> While `to_lowercase` will handle basic Unicode, it won’t be 100% accurate. If we were writing a real application, we’d want to do a bit more work here, but this section is about environment variables, not Unicode, so we’ll leave it at that here.

Note that `query` is now a `String` rather than a `string slice`, **because calling `to_lowercase` creates new data rather than referencing existing data.** Say the query is "rUsT", as an example: that string slice doesn’t contain a lowercase u or t for us to use, so we have to allocate a new `String` containing "rust". **When we pass query as an argument to the contains method now, we need to add an ampersand because the signature of contains is defined to take a string slice.**

```rust
pub struct Config {
    // --snip--
    pub ignore_case: bool;
}

// change run function
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insentitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    }

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// check for env
use std::env;

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

The `is_ok` method on the `Result` to check whether the environment variable is set, which means the program should do a case-insensitive search. If the `IGNORE_CASE` environment variable isn’t set to anything, `is_ok` will return false and the program will perform a case-sensitive search. **We don’t care about the value of the environment variable, just whether it’s set or unset, so we’re checking `is_ok` rather than using `unwrap`, `expect`, or any of the other methods we’ve seen on `Result`.**

```sh
cargo run -- to poem.txt

IGNORE_CASE=1 cargo run -- to poem.txt
```

## Writing error messages to standard error instead of standard output

At the moment, we’re writing all of our output to the terminal using the `println!` macro. In most terminals, there are two kinds of output: **standard output (stdout)** for general information and **standard error (stderr)** for error messages. **This distinction enables users to choose to direct the successful output of a program to a file but still print error messages to the screen.**

The `println!` macro is only capable of printing to standard output, so we have to use something else to print to standard error.

### Checking where errors are written

Command line programs are expected to send error messages to the standard error stream so we can still see error messages on the screen even if we redirect the standard output stream to a file. Our program is not currently well-behaved: we’re about to see that it saves the error message output to a file instead!

```sh
# the > tells the shell to write the contents of standard output to output.txt instead of the screen
cargo run > output.txt
```

### Printing errors to standard error

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
```

```sh
# The error onscreen and `output.txt` contains nothing
cargo run > output.txt

# wont see any output to the terminal, and output.txt will contain our results
cargo run -- to poem.txt output.txt
```
