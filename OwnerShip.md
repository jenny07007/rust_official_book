#

**Ownership** is a set of rules that governs how a Rust program manages memory.

**Stack** -- last in, first out. Adding data is called 'pushing onto the stack', and removing data is called 'popping off the stack.'

**Heap** -- less organized.The momoyr allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a **pointer**, which is the addrss of that location. This process is called **allocating on the heap** or **allocating**.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data.
Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.

## Ownership rules

- Each value in Rust has a variable that's called its **owner**.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## The string type

```rs
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

```rs
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

```rs
// s1 was moved to s2
let s1 = String::from("hello");
let s2 = s1; // s2 is a reference to s1

println!("{}, world!", s1); // does not work
```

With only `s2` valid, when it goes out of scope, it alone will free the memory, and we’re done.

In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. Therefore, any _automatic_ copying can be assumed to be inexpensive in terms of runtime performance.

## Clone

Deeply copy the heap data of the `String`. When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive.

```rs
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

## Copy - Stack-only data

If a type implements the `Copy` trait, a variable is still valid after assignment to another variable. Rust won’t let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait. If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error.

```rs
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

```rs
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

```rs
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

```rs
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
