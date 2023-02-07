#

- [](#)
  - [Writes tests](#writes-tests)
    - [The `assert!` marco](#the-assert-marco)
    - [The `assert_eq!` and `assert_ne!` marcos](#the-assert_eq-and-assert_ne-marcos)
    - [Checking for panics with `should_panic`](#checking-for-panics-with-should_panic)
    - [Using `Result<T, E>` in tests](#using-resultt-e-in-tests)
  - [Controlling how test are run](#controlling-how-test-are-run)
    - [Running tests in parallel or consecutively](#running-tests-in-parallel-or-consecutively)
  - [Showing function output](#showing-function-output)
  - [Runnung a subset of tests by name](#runnung-a-subset-of-tests-by-name)
  - [Filtering to run multiple tests](#filtering-to-run-multiple-tests)
  - [Ignoring some tests unless specifically required](#ignoring-some-tests-unless-specifically-required)
  - [Test organization](#test-organization)
    - [Unit tests](#unit-tests)
      - [Testing Private Functions](#testing-private-functions)
    - [Integration Tests](#integration-tests)
    - [Submodules in Integration Tests](#submodules-in-integration-tests)
    - [Integration Tests for Binary Crates](#integration-tests-for-binary-crates)

## Writes tests

The bodies of test functions typically perform three actions:

1. set up any needed data or state
2. Run the code you want to test
3. Assert the results are what you expect

The features Rust provides specifically for writing tests that take these actions, which included the `test` attribute, a few macros, and the `should_panic` attribute.

[adder lib project](./adder/)

```sh
cargo new adder --lib
```

```rust
// indicates this is a test function
#[cfg(test)]
mod tests {
    use super::*;
    // non-test function in the test module
    #[test]
    fn it_works() {
        let result = add(2, 2);
        // assert_qu! marco to assert that `result` ☝️
        assert_eq!(result, 4);
    }
}
```

```sh
cargo test
```

`Doc-tests adder` is for the results of any documentation tests. Rest can compile any code examples that appear in our API documentation.This feature helps keep your docs and your code in sync!

### The `assert!` marco

Is useful when you want to ensure that some condition in a test evaluates to `true`. We give the `assert!` marco an argument that evaluates to a Boolean. If the value is `true`, nothing happens and the test passes. If the value is `false`, the `assert!` marco calls `panic!` to cause the test to fail.

```rust
// because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module.
// we use a glob here so anything we define in the outer module is available toe this `tests` module
mod tests {
    use super::*;
    // ---
}
```

### The `assert_eq!` and `assert_ne!` marcos

The `assert_ne!` macro will pass if the two values we give it are not equal and fail if they're equal. This marco is most useful when we're not sure what a value **will** be, but we know what the value definitely **shouldn't** be.

Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators `==` and `!=`, respectively. When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the `PartialEq` and `Debug` traits. All primitive types and most of the standard library types implement these traits. For structs and enums that you define yourself, you’ll need to implement `PartialEq` to assert equality of those types. You’ll also need to implement `Debug` to print the values when the assertion fails. Because both traits are derivable traits, this is usually as straightforward as adding the `#[derive(PartialEq, Debug)]` annotation to your struct or enum definition.

### Checking for panics with `should_panic`

To make `should_panic` test more percise, we can add an optional `expected` parameter to the `should_panic` attribute.

```rust
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        }
        Guess { value }
    }
}

#[cfg(test)]
mod test {
  use super::*;

  // will fail. because the value we passed is greater than 100,
  // but the panic message doesn't include the string we expected.
  #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

### Using `Result<T, E>` in tests

The `it_works` function now has the `Result<(), String>` return type, which enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant.

We can not use `#[should_panic]` annotation on tests that use `Result<T, E>`. To assert that an operation returns an `Err` variant, don't use the question mark operator on the `Result<T, E>` value. Instead, use `assert!(value.is_err())`.

```rust
#[cfg(test)]
mod tests {
  #[test]
  fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }
}
```

## Controlling how test are run

```sh
cargo test
cargo test --help
```

### Running tests in parallel or consecutively

When you run multiple tests, by default they run in parallel using threads, meaning they finish running faster and you get feedback qucker.

```sh
# don't want to run tests in parallel or
# want more fine-grained control over the number of threads used
# we set the number of threads to 1, telling the program not to use any parallelism.
cargo test -- -- test-threads=1
```

Ruung the tests using one thread will take longer than running them in parallel, but the tests won't interface with each other if they share state.

## Showing function output

```sh
cargo test -- --show-output
```

## Runnung a subset of tests by name

```sh
cargo test [test name]
```

## Filtering to run multiple tests

```sh
# add_one() {}
# add_two() {}
cargo test add
```

## Ignoring some tests unless specifically required

```rust
#[test]
fn it_works() {
  assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test(){//...}
```

```sh
# run only the ignored tests
cargo test -- --ignored

# run all tests whether they're ignored or not
cargo test -- --include-ignored
```

## Test organization

### Unit tests

The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`.

The attribute `cfg` stands for **configuration** and tells Rust that the following item should only be included given a certain configuration option. In this case, the configuration option is `test`, which is provided by Rust for compiling and running tests. By using the `cfg` attribute, Cargo compiles our test code only if we actively run the tests with `cargo test`. This includes any helper functions that might be within this module, in addition to the functions annotated with `#[test]`.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

#### Testing Private Functions

There’s debate within the testing community about whether or not private functions should be tested directly, and other languages make it difficult or impossible to test private functions. Regardless of which testing ideology you adhere to, Rust’s privacy rules **do** allow you to test private functions.

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

Note that the `internal_adder` function is not marked as pub. Tests are just Rust code, and the `tests` module is just another module. As we discussed, **items in child modules can use the items in their ancestor modules.** **In this test, we bring all of the test module’s parent’s items into scope with use `super::*`, and then the test can call `internal_adder`.** If you don’t think private functions should be tested, there’s nothing in Rust that will compel you to do so.

### Integration Tests

**In Rust, integration tests are entirely external to your library.** They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API. Their purpose is to test whether many parts of your library work together correctly. Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well. To create integration tests, you first need a tests directory.

```sh
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

```sh
cargo test --test integration_test
```

### Submodules in Integration Tests

Each file in the `tests` directory is compiled as its own separate `crate`, which is useful for creating separate scopes to more closely imitate the way end users will be using your crate. However, this means files in the `tests` directory don’t share the same behavior as files in `src` do.

```rust
// tests/common.rs
pub fn setup() {
    // setup code specific to your library's tests would go here
}
```

### Integration Tests for Binary Crates

If our project is a **binary crate** that only contains a `src/main.rs` file and doesn’t have a `src/lib.rs` file, we can’t create integration tests in the tests directory and bring functions defined in the `src/main.rs` file into scope with a use statement. **Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.**

This is one of the reasons Rust projects that provide a binary have a straightforward `src/main.rs` file that calls logic that lives in the `src/lib.rs` file. Using that structure, integration tests can test the library crate with use to make the important functionality available. If the important functionality works, the small amount of code in the `src/main.rs` file will work as well, and that small amount of code doesn’t need to be tested.
