#

- [](#)
  - [Ownership rules](#ownership-rules)
  - [The string type](#the-string-type)
  - [Move \&\& Double free error](#move--double-free-error)
  - [Clone](#clone)
  - [Copy - Stack-only data](#copy---stack-only-data)
  - [Ownership and functions](#ownership-and-functions)
  - [Return Values and Scope](#return-values-and-scope)
  - [References and Borrowing](#references-and-borrowing)
  - [Mutable References](#mutable-references)
  - [Dangling References](#dangling-references)
  - [The rules of References](#the-rules-of-references)
  - [The slice type](#the-slice-type)
  - [String slices](#string-slices)
  - [String literals are slices](#string-literals-are-slices)
  - [String slices as parameters](#string-slices-as-parameters)
  - [Other slices](#other-slices)
  - [Summary](#summary)

|                         | Pros                                                                                                       | Cons                                                                                                         |
| ----------------------- | ---------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| Garbage collection      | **• Error free** <br/> • Faster write time                                                                 | • No control over memory <br /> • Slower and unpredicatable runtime performance <br /> • Larger program size |
| Manual memory mangement | • Control over memory <br /> • Faster runtime<br /> • No Smaller program size <br />                       | • Error prone <br /> • Slower write time <br />                                                              |
| Ownership model         | • Control over memory <br /> **• Error free** <br /> • Faster runtime <br /> • Smaller program size <br /> | • Slower write time. Learning curve (fighting with the borrow checker)                                       |

**Ownership** is a set of rules that governs how a Rust program manages memory.

**Stack** -- last in, first out. Adding data is called 'pushing onto the stack', and removing data is called 'popping off the stack.'

**Heap** -- less organized.The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a **pointer**, which is the addrss of that location. This process is called **allocating on the heap** or **allocating**.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data.
Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.

## Ownership rules

- Each value in Rust has a variable that's called its **owner**.
- **There can only be one owner at a time.**
- **When the owner goes out of scope, the value will be dropped.**

## The string type

```rust
// a string from a string literal using `from`
// immutable string literal - hardcode into the final excutable
let s = String::from("hello");

// mutalbe String type - allocate an amount of memory on the heap
let mut h = String::from("hello");
// appends a literal to a String
h.push_str(", world!");
```

- The memory must be requested from the memory allocator at runtime
- We need a way of returning this memory to the allocator when we’re done with our String.

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.

```rust
{
  let s = String::from("hello"); // s is valid from this point forward
  // do stuff with s
}
// this scope is now over, and s is no longer valid
```

Rust calls the `drop` function automatically at the closing curly bracket, and cleans up the heap memory for that variable.

## Move && Double free error

when s2 and s1 go out of scope, they will both try to free the same memory.
To ensure memory safety, after the line let `s2` = `s1`, Rust considers `s1` as no longer valid. Therefore, Rust doesn’t need to free anything when `s1` goes out of scope.

```rust
// s1 was moved to s2
let s1 = String::from("hello");
let s2 = s1; // s2 is a reference to s1

println!("{}, world!", s1); // does not work
```

With only `s2` valid, when it goes out of scope, it alone will free the memory, and we’re done.

In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. Therefore, any _automatic_ copying can be assumed to be inexpensive in terms of runtime performance.

## Clone

Deeply copy the heap data of the `String`. When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

## Copy - Stack-only data

If a type implements the `Copy` trait, a variable is still valid after assignment to another variable. Rust won’t let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait. If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error.

```rust
let x = 5; // x is on the stack
let y = x;

println!("x = {}, y = {}", x, y);
```

Here are some of the types that implement Copy:

- All the integer types, such as `u32`.
- The Boolean type, `bool`, with values `true` and `false`.
- All the floating point types, such as `f64`.
- The character type, `char`.
- Tuples, if they only contain types that also implement `Copy`. For example, (`i32`, `i32`) implements `Copy`, but (`i32`, `String`) does not.

## Ownership and functions

If we tried to use `s` after the call to `takes_ownership, Rust would throw a compile-time error. These static checks protect us from mistakes.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

```

## Return Values and Scope

Returning values can also transfer ownership.

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}
```

The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.

```rust
// return multiple values using a tuple
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

But this is too much ceremony and a lot of work for a concept that should be common. Luckily for us, Rust has a feature for using a value without transferring ownership, called **references**.

## References and Borrowing

A reference is like a pointer in that it's an address we can follow to access data stored at that address that is owned by some other variable.
Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type.

```rust
fn main() {
    let s1 = String::from("hello");

  // the ampersand (&) represent references, and they allow you to refer
  // to some value without taking ownership of it.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// s is a reference to a String
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.
```

> **Note:** The opposite of referencing by using the ampersand (&) is dereferencing, which is done by using the asterisk (\*).

We call the action of creating a reference **borrowing**. We are not allowed to modify the value that the reference points to.

## Mutable References

To allow us to modify a borrowed value.

```rust
fn main() {
    // create a mutable reference with `&mut s` where we call the `change` function
    let mut s = String::from("hello");

    change(&mut s);
}
// accept a mutable reference with `some_string`
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Mutable reference have one big restriction: you can have one mutable reference to a particular piece of data at a time. The benefit of having this restriction is that Rust can prevent data races at complie time. A `data race` is similar to rece condition and happens when these three behaviours occur.

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data
- There's no mechanism being used to synchronize access to the data.

```rust
let mut s = String::from("hello");

// use curly brackers to create a create a new scope
{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

We cannot have a mutable reference while we have an immutable one to the same value

```rust
let mut s = String::from("hello");
let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM
println!("{}, {}, and {}", r1, r2, r3);
```

```rust
let mut s = String::from("hello");
let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
let r3 = &mut s; // no problem
println!("{}", r3);
```

## Dangling References

`dangling pointer` -- a pointer that references a location in memory that may have been given to someone else -- by freeing some moemory while preserving a pointer to that memory.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger! Danger!
```

```rust
// this works without any problems. Ownership is moved out, and nothing is deallocated
fn no_dangle() -> String{
  let s = String::from("hello");
  s
}
```

## The rules of References

- At any given time, you can have **either** one mutable reference **or** any number of immutable references.
- References must always be valid.

## The slice type

A slice is kind of reference, so it does not have ownership

```rust
fn  first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert String to an array of bytes

    // iterate -> `iter` is a method that returns each element in a collection and
    // that `enumerate` wraps the result of `iter` and returns each element as part of a tuple insted.
    // becuase the  `emuerate` method returns a tuple, we can use patterns to destructure that tuple
    for (i, &item) in bytes.iter().enumerate() {
        // we specify a pattern that has `i` for the index in the tuple and `&item` for the single byte in the tuple.
        // because we get a reference to tje element from `.iter().enumerate()`, we use `&` in the pattern.
        // we search for the byte that represents the space by using the byte literal syntax.
        // if we find a space, we return the position. otherwise, we return the length of the string by using `.len()`.
        if item == b' ' {
            return i;
        }
    }

    s.len()
```

We are returning a usize on its own, but it's only a meaningful number on the context of the `&String`. Because it's a separate value from the `String`, there's no guarantee that it will still be valid in the future

## String slices

```rust
let s = String::from("hello world");

let hello = &s[0..5]; // a reference to a portion of the String
// or let hello = &s[..5];
let world = &s[6..11];
```

```rust
let s = String::from("hello");
let len = s.len();  // 5
let slice = &s[3..len]; // lo
// or let slice = &s[3..];
```

```rust
let s = String::from("hello");
let len = s.len();  // 5
let slice = &s[0..len]; // hello
let slice = &s[..]; // hello
```

> String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, you’ll get an error.

```rust
fn main() {

    // if we have an immuatable referebce to something, we cannot also take a mutable reference.
    // because `clear` needs to truncate the String, it needs to get a mutable reference
    let s = String::from("hello world");

    let w = first_word(&s);
    s.clear(); // slice makes this bug impossibe. -> error

    println!("first word: {}", w);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
```

## String literals are slices

The type of `s` here is `&str`: it's a slice pointing to the specific point of the binary.That's why string literals are immutable; `&str` is a an immutable reference.

```rust
let s = "hello world";
```

## String slices as parameters

```rust
fn first_word(s: &String) -> &str { }

// more experienced. because it allows us to use the same function on both `&String` values and `&str` values.
// if we have a string slice, we can pass it in directly. if we have a `String`, we can pass a slice of the `String` or a reference to the `String`. --> `deref coercions`
fn first_word(s: &str) -> &str{ }
```

```rust
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s,
    // which are equivalent to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // because string literals *are* string slices already,
    // this works without the slice syntax
    let word = first_word(my_string_literal);

    println!("first word: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
```

## Other slices

The slice has the type `&[i32]`.It works the same way as string slices do, by storing a reference to the first element and a length.

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);
```

## Summary

The concept of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don't have to write and debug extra code to get this control.
