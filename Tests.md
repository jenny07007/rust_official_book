# Writes tests

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

## The `assert!` marco

Is useful when you want to ensure that some condition in a test evaluates to `true`. We give the `assert!` marco an argument that evaluates to a Boolean. If the value is `true`, nothing happens and the test passes. If the value is `false`, the `assert!` marco calls `panic!` to cause the test to fail.

```rust
// because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module.
// we use a glob here so anything we define in the outer module is available toe this `tests` module
mod tests {
    use super::*;
    // ---
}
```

## The `assert_eq!` and `assert_ne!` marcos

The `assert_ne!` macro will pass if the two values we give it are not equal and fail if they're equal. This marco is most useful when we're not sure what a value **will** be, but we know what the value definitely **shouldn't** be.

Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators `==` and `!=`, respectively. When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the `PartialEq` and `Debug` traits. All primitive types and most of the standard library types implement these traits. For structs and enums that you define yourself, you’ll need to implement `PartialEq` to assert equality of those types. You’ll also need to implement `Debug` to print the values when the assertion fails. Because both traits are derivable traits, this is usually as straightforward as adding the `#[derive(PartialEq, Debug)]` annotation to your struct or enum definition.

## Checking for panics with `should_panic`

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

## Using `Result<T, E>` in tests

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
