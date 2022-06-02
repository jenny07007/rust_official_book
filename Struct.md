#

- [Using strcts to structure related data](#using-strcts-to-structure-related-data)
- [Defining and instantiating structs](#defining-and-instantiating-structs)
- [Creating instances from other instances with struct update syntax](#creating-instances-from-other-instances-with-struct-update-syntax)
- [Using tuple structs without named fields to create different types](#using-tuple-structs-without-named-fields-to-create-different-types)
- [Unit-like structs without any fields](#unit-like-structs-without-any-fields)
- [Ownership of struct data](#ownership-of-struct-data)
- [examples](#examples)
- [Summary](#summary)

## Using strcts to structure related data

A struct or structure is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

A `struct` is like an object's data attributes.

## Defining and instantiating structs

```rs
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

fn main() {
  let mut user1 = User {
    active: true,
    username: String::from("John"),
    email: String::from("john@helloworld.com"),
    sign_in_count: 1,
  }

  // mutable instance
  user1.email = String::from("john_work@helloworld.com");
}

// return a User instance with given email & username
fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
```

## Creating instances from other instances with struct update syntax

The `struct` update syntax users = like an assignment. because it moves the data.
In this example, we can no longer user `user1` after creating `user2` because the `String` in the `username` field of `user1` was moved into `user2`.
If we had given `user2` nwe `String` values for both `email` and `username`, and thus only used the `active` and `sign_in_count` values from `user1`, the `user1` would still be valid after creating `user2`.
The types of `active` and `sign_in_count` are types that implement the `Copy` trait.

```rs
fn main() {
  let user2 = User {
    email: String::from("user2@hello.com"),
    ..user1
}
```

## Using tuple structs without named fields to create different types

**Tuple structs** - have the added meaning the struct name provides but don't have names associated with their fields, rather they just have the types of the fields.
Truple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.

```rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
  // different types
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}
```

## Unit-like structs without any fields

Unit-like structs are useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself.

```rs
// struct that does not have fields
struct AlwaysEqual;

fn main() {
  let subject = AlwaysEqual;
}
```

## Ownership of struct data

Lifetimes - ensure that the data referenced by a struct is valid for as long as the struct is.

```rs
struct User {
  username: &str,
  email: &str,
  active: bool,
  sign_in_count: u64,
}

fn main() {
  let user1 = User {
    username: "John",
    email: "xxx.com",
    active: true,
    sign_in_count: 1,
  }
}
```

## examples

[struct examples](./rectangles/)

## Summary

structs let you create custom types that are meaningful for your domain.By using structs, you can keep associated pieces of data connected to each piece to make you code clear. In `impl`, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior of your structs have.
