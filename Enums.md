#

- [](#)
  - [Enums - Enumerations](#enums---enumerations)
  - [Defining an enum](#defining-an-enum)
  - [Enum values](#enum-values)
  - [Option enum](#option-enum)
  - [The `match` Control Flow Construct](#the-match-control-flow-construct)
  - [Patterns that bind to values](#patterns-that-bind-to-values)
  - [Matching with`Option<T>`](#matching-withoptiont)
  - [matches are exhaustive](#matches-are-exhaustive)
  - [`catch-all` pattern and the `_` placeholder](#catch-all-pattern-and-the-_-placeholder)
  - [Concise Control Flow with `if let`](#concise-control-flow-with-if-let)

## Enums - Enumerations

Allows us to define a type by enumerating its possible **variants**. Rust's enums are most similar to **algebraic data types** in functional languages.

## Defining an enum

```rust
enum IpAddrKind {
  V4,
  V6,
}
```

## Enum values

```rust
// instances of each of the two variants of IpAddrKind
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

fn route(ip_kind: IpAddrKind) {}

// call the function with each of the enum values
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

Enum + Struct

```rust
enum IpAddrKind {
  V4,
  V6
}

struct IpAddr {
  kind: IpAddrKind::V4,
  address: String::from("127.0.0.1")
}

let home = IpAddr {
  kind: IpAddrKind::V4,
  address: String::from("127.0.0.1")
}

let loopback = IpAddr {
  kind: IpAddrKind::V6,
  address: String::from("::1")
}
```

Advantage to use an enum rather than a struct: each variant can have different types and amounts of associated data.

```rust
enum IpAddr {
  V4(String),
  V6(String),
}

// Version four type IP address will always have four numeric components that will have values between 0 and 255.
// struct does not allow storing `v4` address as `u8` values but still express `v6` address as one `String` value.
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

// IpAddr::V4 is a function call that takes a String and returns an instance of the IpAddr type
// we automaticlly get this constructor function defined as a result of defining the enum
let home = IpAddr::V4(String::from("127.0.0.1"))
let loopback = IpAddr::V6(String::from("::1"))
```

You can put any kind of data inside an enum variant, even another enum.

```rust
enum Message{
  Quit, // has no data associated with it at all
  Move {x: i32, y: i32}, // has named fields like a struct does
  Write(String), // includes a single `String`
  ChangeColor(i32, i32, i32), // includes three `i32` values
}
```

Define methods on enums

```rust
impl Message {
  fn call(&self) {
    // method body would be defined here
  }
}

let m = Message::Write(String::from("Hello"));
m.call();
```

## Option enum

The `Option` type encodes the very common scenario in which a value could be something or it could be nothing.

Rust does not have the null feature that many other languages have.

`Null` is a value that means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.

The problem with null is that if you try to use a null value as a not-null value, you'll get an error of some kind. Because this null or not-null property is pervastive. The concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.

```rust
// T, a generic type parameter,it means the Some variant of the Option enum can hold one piece of data of any type, and that each concrete type that gets used in place of T makes the overall Option<T> type a different type.
enum Option<T> {
  None,
  Some(T),
}

let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;
```

## The `match` Control Flow Construct

Think of a `match` expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into.

```rust
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

// we list the `match` keyword followed by an expression, which in this case is the value `coin`.
// if -> must return a boolean value. with match -> we can return any type,
fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin:: Penny => {
      println!("Lucky penny!");
      1
    },
    Coin:: Nickel => 5,
    Coin:: Dime => 10,
    Coin:: Quarter => 25,
  }
}
```

## Patterns that bind to values

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
  Alabama,
  Alaska,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin:: Penny => 1,
    Coin:: Nickel => 5,
    Coin:: Dime => 10,
    Coin:: Quarter(state) => {  // binding to a value
      println!("State quarter from {:?}!", state);
      25
    },
  }
}

// call value_in_cents
value_in_cents(Coin::Quarter(UsState::Alaska));
```

## Matching with`Option<T>`

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

Combining `match` and enums is useful in many situations. You'll see this parttern a lot in Rest code: `match` against an enum, bind a variable to the data inside, and then execute code based on it.

## matches are exhaustive

We must exhaust every last possibility in order for the code to be valid. Especially in the case of `Option<T>`, when Rust prevents us from forgetting to handle the `None` case, it protects us from assuming that we have a value when we might have null.

```rust
// will run error, because we did not handle the None case
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    Some(i) => Some(i + 1),
  }
}
```

## `catch-all` pattern and the `_` placeholder

This `catch-all` pattern meets the requirment that `match` must be exhaustive. Note that we have to put the catch-all arm last because the patterns are evaluated in order. Rust will warn us if we add arms after a catch-all because those later arms would never match.

`_` is a special pattern that matches any value and does not bind to that value. This tells Rust we aren't going to use the value, so Rust won't warn us about an unused variable.

`()` the empty tuple value here we tells Rust explicitly that we aren't going to use any other value that doesn't match a pattern in an earlier arm, and we don't want to run any code in this case.

```rust
let dice_roll = 9
// catch-all pattern
match dice_roll {
  3 => add_fancy_hat(),
  7 => remove_fancy_hat(),
  other => move_player(other)
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

// _
match dice_roll {
  3 => add_fancy_hat(),
  7 => remove_fancy_hat(),
  _ => roll_again()
}

fn roll_again() {}

// the empty tuple type
match dice_roll {
  3 => add_fancy_hat(),
  7 => remove_fancy_hat(),
  _ => ()
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

## Concise Control Flow with `if let`

The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest.

if the value is `Some`, we print out the value in the `Some` variant by binding the value to the variable `max` in the pattern. We don't want to do anything with the `None` value. To satisfy the `match` expression, we have to add `_ => ()` after processing just one variant, which is annoying.

The `if let` syntax takes a pattern and expression separated by an equal sign. It works the same way as a `match`, where the expression is given to the `match` and the pattern is its first arm.

Using `if let` means less typing, less indentation, and less boilerplate code. However you lose the exhaustive checking the `match` enforces. Choosing between `match` and `if let` depends on what you're doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

You can think of `if let` as syntax sugar for a `match` that runs code when the value matches one pattern and then ignores all other values.

```rust
let config_max = Some(100);
match config_max{
  Some(max) => println!("The maximum is configured to {}", max),
  _ => ()
}

// if let
if let Some(max) = config_max{
  println!("The maximum is configured to {}", max);
}

// count all non-quarter coins with match
let mut count = 0
match coin {
  Coin::Quarter(state) => println!("State quarter from {:?}!", state),
  _ => count += 1
}
// with if let & else
if let Coin::Quarter(state) = coin {
  println!("State quarter from {:?}", state);
} else {
  count += 1
}
```
