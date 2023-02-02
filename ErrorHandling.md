#

- [](#)
  - [Unrecoverable Errors with `panic!`](#unrecoverable-errors-with-panic)
  - [Recoverable Errors with `Result`](#recoverable-errors-with-result)
  - [Matching on different Errors](#matching-on-different-errors)
    - [Alternatives to using `match` wuth `Result<T, E>`](#alternatives-to-using-match-wuth-resultt-e)
  - [Shortcuts for panic on error: `unwrap` and `expect`](#shortcuts-for-panic-on-error-unwrap-and-expect)
  - [Propagating Error](#propagating-error)
    - [A shortcut for propagating error: the `?` operator](#a-shortcut-for-propagating-error-the--operator)
  - [Where the `?` operator can be uses](#where-the--operator-can-be-uses)
  - [To panic! or Not to panic!](#to-panic-or-not-to-panic)
    - [Example, prototype code, and tests](#example-prototype-code-and-tests)
    - [Cases in which you have more informaton than the compiler](#cases-in-which-you-have-more-informaton-than-the-compiler)
  - [Guidelines for error handling](#guidelines-for-error-handling)
  - [Creating custom types for validation](#creating-custom-types-for-validation)

## Unrecoverable Errors with `panic!`

There are two ways to cause a panic in practice: by taking an action that causes our code to panic (such as accessing an array past the end) or by explictly calling the `panic!` macro.

By default, when a panic occurs, the program starts **unwinding**, which means Rust walks back up the stack and cleans up the data from each function it encourters. However, this walking back and cleanup is a lot of work. Rust allows you to choose the alternative of immediately **aborting**, which ends the program without cleaning up.

Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding `panic = "abort"` to the appropriate `[profile]` sections in your `Cargo.toml` file.

```rust
[profile.release]
panic = "abort"
```

```rust
fn main() {
  panic!("crash and burn");
}

// use a panic backtrace
fn main() {
  let v = vec![1,2,3];

  v[99];
}
```

In C, attempting to read beyond the end of a data structure is undefined behaviour. You might get whatever is at the location in memory that would correspond to that element in the data structure, even though the memory doesn't belong to that structure. This is called `buffer overread` and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn't be allowed to taht is stored after the data structure.
To protect your program from this sort of vulnerability, if you try to read an element at an index that doesn't exist, Rust will stop execution and refuse to contiune.

```sh
RUST_BACKTRACE=1 cargo run
```

## Recoverable Errors with `Result`

The `T` and `E` are generic type parameters. `T` represents the type of the value that will be returned in a success case with the `Ok` variant, and `E` represents the type of the error that will be returned in a failure case within the `Err` variant.

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

Like the `Option` enum, the `Result` enum and its variants have been brought into scope by the prelude, so we don't need to specify `Result::` before the `Ok` and `Err` variants in the `match` arms.

```rust
use std::fs::File;

fn main() {
  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };
}
```

## Matching on different Errors

The type of the value that `File::open` returns the `Err` variant is `io::Error`, which is a struct provided by the standard library. The enum `io::ErrorKind` is provided by the standard library and has variant representing the different kinds of errors that might result from an `io` operation. The variant we want to use is `ErrorKind::NotFound`, which indicates the file we're trying to open doesn't exist yet. So we match on `greeting_file_result`, but we also have an inner match on `error.kind()`.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e)
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error)
      }
    }
  };
}
```

### Alternatives to using `match` wuth `Result<T, E>`

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      // expression, no ; in the end
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error);
    }
  });
}
```

## Shortcuts for panic on error: `unwrap` and `expect`

The `Result<T, E>` type has many helper methods defined on it to do various, more specific tasks. The `unwrap` method is a shortcut method implemented just like the `match` expression. If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will call the `panic!` marco for us.

```rust
use std::fs::File;

// with unwrap
fn main() {
  let greeting_file = File::open("hello.txt").unwrap();
}

// with expect
fn main() {
  let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
}
```

The `expect` method lets us also choose the `panic!` error message. Using `expect` instead of `unwrap` and providing good error messages can convey your intent and make tracking down the source of a panic easier.

In production-quality code, most Rustaceans choose `expect` rather than `unwrap` and give more context about why the operation is expected to always succeed. That way, if your assumptions are ever proven wrong, you have more information to use in debugging.

## Propagating Error

When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do. This is known as **propagating** the error and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.

The `Result<String, io::Error>`. This means the function is returning a value of the type `Result<T, E>` where the generic parameter `T` has been filled in with the concrete type `String`, and the generic type `E` has been filled in with the concrete type `io::Error`.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
  let username_file_result = File::open("hello.txt");

  let mut username_file = match username_file_result {
    Ok(file) => file,
    Err(e) => return Err(e)
  };

  let mut username = String::new();

  match username_file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(e) => Err(e),
  }
}
```

### A shortcut for propagating error: the `?` operator

What the `?` operator does: error values that have the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library, which is used to convert values from one type into another. When the `?` operator calls the `from` function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.

we could change the `read_username_from_file` function to return a custom error type named `OurError` that we define. If we also define `impl From<io::Error>` for `OurError` to construct an instance of `OurError` from an `io::Error`, then the `?` operator calls in the body of `read_username_from_file` will call from and convert the error types without needing to add any more code to the function.

The `?` at the end of the `File::open` call will return the value inside an `Ok` to the variable `username_file`. If an error occurs, the `?` operator will return early out of the whole function and give any `Err` value to the calling code. The same thing applies to the `?` at the end of the `read_to_string` call.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

A shorten version of code

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result(String, io::Error) {
  let mut username = String::new();
  File::open("hello.txt")?.read_to_string(&mut username)?;

  Ok(username)
}
```

Reading a file into a string is a fairly common operation, so the standard library provides the convenient `fs::read_to_string` function that opens the file, creates a new `String`, reads the contents of the file, puts the contents into that `String`, and returns it. Of course, using `fs::read_to_string` doesn’t give us the opportunity to explain all the error handling, so we did it the longer way first.

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt");
}
```

## Where the `?` operator can be uses

The `?` operator can only be used in functions whose return type is compatible with the value the `?` is used on. This is because the `?` operator is defined to perform an early return of a value out of the function.

> This code opens a file, which might fail. The `?` operator follows the `Result` value returned by `File::open`, but this `main` function has the return type of `()`, not `Result`

```rust
//  WON'T COMPILE
use std::fs::File;
fn main() {
  let greeting_file = File::open("hello.txt")?;
}
```

To fix this error, there are two choices. One choice is to change the return type of your function or be compatible with the value you're using the `?` operator on as long as you have no restrictions preventing that. The other technique is to use a `match` or one of the `Result<T, E>` methods to handle the `Result<R, E>` in whatever way is appropriate.

As with using `?` on `Result`, you can only use `?` on `Option` in a function that returns an `Option`. The behavior of the `?` operator when called on an `Option<T>` is similar to its behavior when called on `Result<R, E>`: if the value is `None` will be returned early from the function at that point. If the value is `Some`, the value inside the `Some` is the resulting value of the expression and the function continues.

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
  text.lines().next()?.chars().last()
}
```

> The `?` operator won't automatically convert a `Result` to an `Option` or vice versa; in those cases, you can use methods like the `ok` method on `Result` or the `ok_or` method on `Option` to do the conversation explicity.

`main` can also returns a `Result<(), E>`. We change the return type of `main` to be `Result<(), Box<dyn Error>>` and add a return value `Ok(())` to the end.

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
  let greeting_file = File::open("hello.txt")?;

  Ok(())
}
```

The `Box<dyn Error>` type is a **trait object**. You can read it to mean **any kind of error**. Using `?` on a `Result` value in a `main` function with the error type `Box<dyn Error>` is allowed, because it allows any `Err` value to be returned early. Even though the body of this `main` function will only ever return errors of type `std::io::Error`, by specifying `Box<dyn Error>`, this signature will continue to be correct even if more code that returns other errors is added to the body of `main`.

When a `main` function returns a `Result<(), E>`, the executable will exit with a value of `0` if `main` returns `Ok(())` and will exit with a nonzero value if `main` returns an `Err` value.

Executables written in C return integers when they exit; programs that exit sucessfully return this integer `0`, and programs that error return some integer other than `0`. Rust also returns integers from excutables to be compatible with this convention.

The main function may return any types that implement the `std::process::Termination` [trait](https://doc.rust-lang.org/std/process/trait.Termination.html), which contains a function `report` that returns an `ExitCode`.

## To panic! or Not to panic!

> Returning `Result` is a good default choice when you're defining a function that might fail.
>
> > In situations such as example, prototype code, and tests, it's more appropriate to write code that panic instead of returning a `Result`.

### Example, prototype code, and tests

In examples, it’s understood that a call to a method like `unwrap` that could panic is meant as a placeholder for the way you’d want your application to handle errors, which can differ based on what the rest of your code is doing.

Similarly, the `unwrap` and `expect` methods are very handy when prototyping, before you’re ready to decide how to handle errors. They leave clear markers in your code for when you’re ready to make your program more robust.

If a method call fails in a test, you’d want the whole test to fail, even if that method isn’t the functionality under test. Because `panic!` is how a test is marked as a failure, calling unwrap or expect is exactly what should happen.

### Cases in which you have more informaton than the compiler

It would also be appropriate to call `unwrap` or `expect` when you have some other logic that ensures the `Result` will have an `Ok` value, but the logic isn’t something the compiler understands. You’ll still have a `Result` value that you need to handle: whatever operation you’re calling still has the possibility of failing in general, even though it’s logically impossible in your particular situation. If you can ensure by manually inspecting the code that you’ll never have an `Err` variant, it’s perfectly acceptable to call `unwrap`, and even better to document the reason you think you’ll never have an `Err` variant in the `expect` text.

```rust
use std::net::IpAddr;

let home:: IpAddr = "127.0.0.1"
  .parse()
  .expect("Hardcoded IP address should be valid");
```

We’re creating an `IpAddr` instance by parsing a hardcoded string. We can see that `127.0.0.1` is a valid IP address, so it’s acceptable to use `expect` here. However, having a hardcoded, valid string doesn’t change the return type of the `parse` method: we still get a `Result` value, and the compiler will still make us handle the `Result` as if the `Err` variant is a possibility because the compiler isn’t smart enough to see that this string is always a valid IP address. If the IP address string came from a user rather than being hardcoded into the program and therefore **did** have a possibility of failure, we’d definitely want to handle the `Result` in a more robust way instead. Mentioning the assumption that this IP address is hardcoded will prompt us to change `expect` to better error handling code if in the future, we need to get the IP address from some other source instead.

## Guidelines for error handling

It's advisable to have your code panic when it's possible that your code could end up in a **bad state**.

- The **bad state** is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
- Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
- There’s not a good way to encode this information in the types you use.

If someone calls your code and passes in values that don’t make sense, it’s best to return an error if you can so the user of the library can decide what they want to do in that case. However, in cases where continuing could be insecure or harmful, the best choice might be to call `panic!` and alert the person using your library to the bug in their code so they can fix it during development. Similarly, `panic!` is often appropriate if you’re calling external code that is out of your control and it returns an invalid state that you have no way of fixing.

However, when failure is expected, it’s more appropriate to return a `Result` than to make a `panic!` call. Examples include a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit. **In these cases, returning a `Result` indicates that failure is an expected possibility that the calling code must decide how to handle.**

When your code performs an operation that could put a user at risk if it’s called using invalid values, your code should verify the values are valid first and `panic` if the values aren’t valid. This is mostly for safety reasons: **attempting to operate on invalid data can expose your code to vulnerabilities**. This is the main reason the standard library will call `panic!` if you attempt an out-of-bounds memory access: trying to access memory that doesn’t belong to the current data structure is a common security problem.

Functions often have contracts: their behavior is only guaranteed if the inputs meet particular requirements. **Panicking when the contract is violated makes sense because a contract violation always indicates a caller-side bug and it’s not a kind of error you want the calling code to have to explicitly handle.** In fact, there’s no reasonable way for calling code to recover; the calling programmers need to fix the code. Contracts for a function, especially when a violation will cause a panic, should be explained in the API documentation for the function.

If your function has a particular type as a parameter, you can proceed with your code’s logic knowing that the compiler has already ensured you have a valid value. For example, **if you have a type rather than an Option, your program expects to have something rather than nothing.** Your code then doesn’t have to handle two cases for the Some and None variants: it will only have one case for definitely having a value. Code trying to pass nothing to your function won’t even compile, so your function doesn’t have to check for that case at runtime. **Another example is using an unsigned integer type such as `u32`, which ensures the parameter is never negative.**

## Creating custom types for validation

```rust
// i32 instead of only u32 to allow potentially nagative numbers
loop {
  // ---
  let guess: i32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => contiune,
  };

  if guess < 1 || guess > 100 {
    println!("The secret number will be between 1 and 100.");
    contiune;
  }
  match guess.cmp(&secret_number) {
    // ---
  }
}
```

Make a new type and put the validation in a function to create an instance of the type rather than repeating the validation everywhere. That way, it's safe for functions to use the new type in their signatures and confidently use the values they receive.

```rust
// Define a `Guess` type that will only create an instance of Guess if the `new` function receive a value between 1 and 100.
pub struct Guess {
  value : i32;
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }
    Guess {value}
  }

// implement a value method that borrows self, it doesn't have any other parameters, and return an `i32`.
// getter -- the purpose is to get some data from its fields and return it.
  pub fn value(&self) -> i32 {
    self.value
  }
}
```

If `value` doesn’t pass this test, we make a `panic!` call, which will alert the programmer who is writing the calling code that they have a bug they need to fix, because creating a `Guess` with a `value` outside this range would violate the contract that `Guess::new` is relying on. The conditions in which `Guess::new` might panic should be discussed in its public-facing API documentation. If `value` does pass the test, we create a new `Guess` with its value field set to the value parameter and return the `Guess`.

This `public` method is necessary because the value field of the `Guess` struct is private. **It’s important that the value field be private so code using the `Guess` struct is not allowed to set value directly**: code outside the module must use the `Guess::new` function to create an instance of `Guess`, thereby ensuring there’s no way for a `Guess` to have a value that hasn’t been checked by the conditions in the `Guess::new` function.

A function that has a parameter or returns only numbers between 1 and 100 could then declare in its signature that it takes or returns a `Guess` rather than an `i32` and wouldn't need to do any additional checks in its body.
