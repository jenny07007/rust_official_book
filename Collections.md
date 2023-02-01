- [Collections](#collections)
  - [Vector](#vector)
    - [Reading elemenets of vectors](#reading-elemenets-of-vectors)
    - [Iterating over the values in a Vector](#iterating-over-the-values-in-a-vector)
    - [Using an enum to store multiple types](#using-an-enum-to-store-multiple-types)
    - [Dropping a vector drops its elements](#dropping-a-vector-drops-its-elements)
  - [String](#string)
    - [Updating a string](#updating-a-string)
    - [Concatenation with the `+` or the `format!` macro](#concatenation-with-the--or-the-format-macro)
    - [Indexing into strings](#indexing-into-strings)
    - [Sliceing Strings](#sliceing-strings)
    - [Methods for iterating over strings](#methods-for-iterating-over-strings)
  - [Hashmap](#hashmap)
    - [HashMaps and Ownership](#hashmaps-and-ownership)
    - [Updating a hash map](#updating-a-hash-map)
    - [Adding a key and value only if a key isn't present](#adding-a-key-and-value-only-if-a-key-isnt-present)
    - [Updating a value based on the old value](#updating-a-value-based-on-the-old-value)
    - [Hash functions](#hash-functions)

# Collections

- can contain multiple values
- the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.
- three collections that are used very often in Rust programs: `vector`, `string`, and `hashmap`

## Vector

A vector allows you to store a variable number of values next to each other.

> Vectors are implemented using generics. `Vec<T>`

```rust
// create a new vector with `Vec::new()`
let v: Vec<i32> = Vec::new();

// create a new vector with `vec!` macro
let v = vec![1,2,3];

// update a vector
let mut v = Vec::new();
v.push(5);
v.push(6);
```

### Reading elemenets of vectors

- via indexing
- use the `get` method

```rust
let v = vec![1,2,3,4,5];

// we use & and [] gives us a reference to the element at the index value.
let third: &i32 = &v[2];
println!("The third element is {third}");

// when we use the `get` method with the index passed as an argument,
// we get an `Option<&T>` thhat we can use with `match`
let third: Option<&i32> = v.get(2);
match third {
  Some(third) => println!("The third element is {third}"),
  None => println!("There is no third element.")
}
```

The reason Rust provides these two ways to reference an element is so you can choose how the program behaves when you try to use an index value outside the range of existing elements.

```rust
let v = vec![1,2,3,4,5];

// panic! This method is best used when you want your program to crash if there's an attempt to access an element past the end of the vector.
let does_not_exist = &v[100];

// returns `None` without panicking.
// Would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances. Your code will then have logic to handle having either `Some(&element)` or `None`
let deos_not_exist = v.get(100);
```

> You cannot have mutable and immutable references in the same scopes

```rust
let mut v = vec![1,2,3,4,5];

// immmutable reference to the first element in a vector
let first = &v[0];

// try to add an element to the end
v.push(6);

// panic!
println!("The first element id : {first}")
```

This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn't enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

### Iterating over the values in a Vector

Iteraring over a vector, whether immutably or mutably, is safe because of the borrow checker's rules. If we attempt to insert or remove items in the `for` loop bodies, we would get a complier errir similar to the last one. The reference to the vector that the `for` loop holds prevents simulataneous modification of the whole vector.

```rust
let v = vec![100, 32, 57];
for i in &v {
  println!("{i}")
}

let mut v = vec![100, 32, 57];

// to change the value that the mutable reference refers to, we have to use the `*` dereference operator to get to the value in `i` before we can use the `+=` operator.
for i in &mut v {
  *i += 50;
}
```

### Using an enum to store multiple types

Vectors can only store values that are the same types. But we can use an enum to represent elements of different types.

```rust
enum SpreadSheetCell {
  Int(i32),
  Float(f64),
  Text(String)
}

let row = vec![
  SpreadSheetCell::Int(3),
  SpreadSheetCell::Text(String::from("blue")),
  SpreadSheetCell::Float(10.12)
]
```

> Rust need to know that types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. We must also be explicit about what types are allowed in this vector. If Rust allowed a vector to hold any type, there would be a chance that one or more types would cause errors with the operations performed on the elements of the vector. Using an enum plus a `match` expression means that Rust will ensure at compile time that every possible case is handled.

### Dropping a vector drops its elements

```rust
{
  let v = vec![1,2,3,4];
  // do stuff with v
} // <- goes out of scope and is freed here
```

## String

A string is a collection of characters.

Rust has only one string type in the core language, which is the **string slice** `str` that is usually seen in its borrowed from `&str`. **String slices** are references to some UTF-8 encoded string data stored elsewhere. **String literals**, are stored in the program's binary and are therefore string slices.

```rust
// create a new string
let mut s = String::new();

// start with a initial string. use `to_string()` method
let data = "initial contents";
let s = data.to_string();
let s = "initial contents".to_string(); // also works

// create a `String` from a string literal
let s = String::from("inital contents");
```

> Strings are UTF-8 encoded.

### Updating a string

> The `push_str` method takes a string slice because we don't necessarily want to take ownership of the parameter.

```rust
let mut s = String::from("foo");
s.push_str("bar");

//
let mut s1 = String::from("foo");
let s2 = "bar";
sl.push_str(s2);
println!("s2 is {s2}"); // works. because push_str doesnt take ownership of s2
```

> The `push` method takes a single character as a parameter and adds it to tje `String`

```rust
let mut s = String::from("lo");
s.push('l'); // lol
```

### Concatenation with the `+` or the `format!` macro

The reason we're able to use `&s2` in the call to `add` is that the compiler can **coerce** the `&String` argument to a `&str`. When we call the `add` method, Rust uses a **deref coercion**, which here turns `&s2` into `&s2[..]`. Because `add` doesn't take ownership of the `s` parameter, `s2` will still be a valid `String` after this operation.

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s2 = s1 + &s2; // s1 has been moved here and can no longer be used
```

```rust
fn add(self, s: &str) -> String {}
```

Second, we can see in the signature that `add` takes ownership of `self`, because `self` doesn't have an `&`. This means `s1` will be removed into the `add` call and will no longer be valid after that. So although `let s3 = s1 + &s2` looks like it will copy both strings and create a new one, this statement acutally takes ownership of `s1`, appends a copy of the contents of `s2`, and then returns ownership of the result. In other words, it looks like it's making a lot of copies but isn't, the implementation is more efficient than copying.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
```

### Indexing into strings

> Rust stirngs do not support indexing.
>
> > **Internal Representation**
> > to avoid returning an unexpected value and causing bugs that might not be discovered immediately.Rust doesn't compile this code at all and prevents misunderstandings early in the development process.
>
> > **Bytes and Scalar Values and Grapheme Clusters**
> > Another point about UTF-8 is there are actually three relevant ways to look at strings from Rust's prespectives: as **bytes**, **scalar values**, and **grapheme cluster** (the closest thing to what we call leters)
>
> > **A final reason Rust doesn't allow us to index into a `String` to get a character** is that indexing operations are expected to always take constant time (O(1)). But it isn't possible to guarantee that performance with a `String`, because Rust would have to walk through the contents from the beginning to the index to determine how mnay valid characters there were.

```rust
// throw error
let s1 = String::from("hello");
let h = s1[0];
```

### Sliceing Strings

Indexing into a string is often a bad idea because it's not clear what the return type of the strings. If you really need to use indices to create string slices, therefore, Rust asks you to be more specific.

```rust
let hello = "Здравствуйте";

// s will be a `&str` that contains the first 4 bytes of the string. Because these characters was 2 bytes, which means s will be Зд
// if we were try to slice only part of a character's bytes with something like `&hello[0..1]`, Rust would panic at runtime.
let s = &hello[0..4];
```

### Methods for iterating over strings

```rust
// use chars method
for c in "Зд".chars() {
    println!("{c}");
}
// or use bytes method
for b in "Зд".bytes() {
    println!("{b}");
}
// 208
// 151
// 208
// 180
```

- iterating grapheme

```toml
[dependencies]
unicode_segmentation = "1.7.1"
```

```rust
use unicode_segmentation::UnicodeSegmentation;

// ["न", "म", "स्", "ते"]
for g: &str in "नमस्ते".graphemes(is_extended: true) {
  println!("{}", g);
}
```

## Hashmap

A hash map allows you to associate a value with a particular key. It's a particular implementation of the more general data structure called a map.

The type `HashMap<K, V>` stores a mapping of keys of types `K` to `V` using a **hashing function**, which determines how it places these keys an values into memory.

Just like vectors, hash maps store their data on the heap. This `HashMap` has keys of type `String` and values of type `i32`. Like vectors, hash maps are hemogeneous: all of the keys must have the same type as each other, and all the values must have the same type.

```rust
// create a new hash map
use std::collections::HashMap;
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// accessing values in a hash map
let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

The `get` method returns an `Option<&V>`, if there's no value for that key in the hash map, `get` will return `None`. This program handles the `Option` by calling `copied` to get an `Option<i32>` rather than an `Option<&i32>`, then `unwrap_or` to set `score` to zero if `scores` doesn't have an entry for the key.

```rust
// iterating over each key/value
for (key, value) in &scores {
  println!("{key}: {value}");
}
```

### HashMaps and Ownership

For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values.

```rust
fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}
```

We aren't able to use the variables `field_name` and `field_value` after they've been moved into the hash map with the call to `insert`.
If we insert references to values into the hash map, the values won't be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid.

### Updating a hash map

```rust
//  overwriting a value
use std::collections::HashMap;
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```

### Adding a key and value only if a key isn't present

The `entry` API takes the key you want to check as a parameter. The return value of the `entry` method is an enum called `Entry` that represents a value that might or might not exist.

The `or_insert` method on `Entry` is defined to return a mutable reference to the value for the corresponding `Entry` key if the key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.

This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

// if there is no value of yellow team, insert the value 50.
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
// {"Yellow": 50, "Blue": 10}
```

### Updating a value based on the old value

We use a hash map with the words as keys and increment the value to keep track of how many times we've seen that word. If it's the first time we've seen a word, we'll first insert the value 0.

The `split_whitespace` method returns an iterator over sub-slices, separated by whitespace, of the value in `text`. The `or_insert` method returns a mutable reference (`&mut V`) to the value for the specified key. Here we store that mutable reference in the `count` variable, so in order to assign to that value, we must first dereference `count` using the asterisk (`*`). The mutable reference goes out of scope at the end of the `for` loop, so all of these changes are safe and allowed by the borrowing rules.

```rust
use std::collections::HashMap;
let text  = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
  let count = map.entry(word).or_insert(0);
  *count += 1;
}

println!("{:?}", map);
// {"wonderful": 1, "hello": 1, "world": 2}
```

### Hash functions

By default, `HashMap` uses a hashing function called **SipHash** that can provide resistance to Denial of Service (DoS) attacks involving hash tables. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. A **hasher** is a type that implements the `BuildHasher` trait. You don’t necessarily have to implement your own hasher from scratch; [crates.io](https://crates.io/) has libraries shared by other Rust users that provide hashers implementing many common hashing algorithms.

https://en.wikipedia.org/wiki/SipHash
