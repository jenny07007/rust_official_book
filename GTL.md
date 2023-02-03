- [Generic Types, Traits, and Lifetimes](#generic-types-traits-and-lifetimes)
  - [Generic Data Types](#generic-data-types)
    - [In function definitions](#in-function-definitions)
    - [In struct Definitions](#in-struct-definitions)
    - [In Enum Definitions](#in-enum-definitions)
    - [In Method Definitions](#in-method-definitions)
    - [Performance of code using generic](#performance-of-code-using-generic)
  - [Traits: Defining shared behavior](#traits-defining-shared-behavior)
    - [Define a trait](#define-a-trait)
    - [Implementing a trait on a type](#implementing-a-trait-on-a-type)
    - [Default implementations](#default-implementations)
    - [traits as parameters](#traits-as-parameters)
    - [Clearer trait bounds with `where` clauses](#clearer-trait-bounds-with-where-clauses)
    - [Returning types that implement traits](#returning-types-that-implement-traits)
    - [Using trait bounds to conditionally implement methods](#using-trait-bounds-to-conditionally-implement-methods)
  - [Validating Reference with Lifetimes](#validating-reference-with-lifetimes)
    - [Preventing dangling references with Lifetimes](#preventing-dangling-references-with-lifetimes)
    - [Borrow checker](#borrow-checker)
    - [Generic lifetimes in functions](#generic-lifetimes-in-functions)
    - [Lifetine annotation syntax](#lifetine-annotation-syntax)
    - [Lifetime annotation in function signatures](#lifetime-annotation-in-function-signatures)
    - [Thinking in terms of lifetimes](#thinking-in-terms-of-lifetimes)
    - [Lifetime annotations in struct definitions](#lifetime-annotations-in-struct-definitions)
    - [Lifetime Elision](#lifetime-elision)
    - [Lifetime annotations in method definitions](#lifetime-annotations-in-method-definitions)
    - [The static lifetime](#the-static-lifetime)
  - [Generic type parameters, trait bounds, and lifetimes together](#generic-type-parameters-trait-bounds-and-lifetimes-together)

---

# Generic Types, Traits, and Lifetimes

- **Generic Types:** reduces code duplication.
- **traits:** combines with generic types to constrain a generic type to accpet only those types that have a particular behavior, as opposed to just any type.
- **lifetimes:** a variety od generics that give the compiler information about how references relate to each oter. Lifetimes allows us to give the compiler enough information about borrowed values so that it can ensure referenecs will be valid in more situations than it could without our help.

```rust
fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let mut largest = &number_list[0];
  for number in &number_list {
    if number > largest {
      largest = number;
    }
  }
  println!("The largest number is {}", largest);
}

// Reduce duplication
fn largest(list: &[i32]) -> &i32 {
  let mut largest = &list[0];
  for item in list {
    if item > largest {
      largest = item
    }
  }
  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {result}");

  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
  let result = largest(&number_list);
  println!("The largest number is {result}");
}
```

> 1. Identify duplicate cade
> 2. Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature
> 3. Update the two instance of duplicatd code to call the function instead

## Generic Data Types

### In function definitions

When defining a function that uses gnerics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value.

```rust
// the `largest` function is generic over some type `T`.
// This function has one parameter named `list`, which is a slice of values of type `T`.
// the `largest` function will return a reference to a value of the same type `T`.
fn largest<T>(list: &[T]) -> &T {
  let mut largest = &list[0];
  for item in list {
    if item > largest {
      largest = item
    }
  }
  largest
}


fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {result}");

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest(&char_list);
  println!("The largest char is {result}");
}

// error[E0369]: binary operation `>` cannot be applied to type `&T`
```

```sh
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
```

The `std::cmp::PartialOrd` is a **trait**. The error states that the body of `largest` won't work for all possible types that `T` could be. Because we want to compare values of type `T` in the body, we can only use types whose values cab be ordered. To enable comparisons, the standard library has the `std::cmp::PartialOrd` trait that you can implement on types. By following the help text's suggestion, we restrict the types valid for `T` to only those that implement `PartialOrd` and it will work, because the standard library implements `PartialOrd` on both `i32` and `char`.

```rust
// solution -> trait
fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
  let mut largest = &list[0];
  for item in list {
    if item > largest {
      larget = item
    }
  }
  largest
}
```

### In struct Definitions

```rust
struct Point<T, U> {
  x: T,
  y: U,
}

fn main() {
  let both_integer = Point {x: 5, y: 10};
  let both_float = Point {x: 1.0, y: 4.0};
  let integer_and_float = Point {x:5, y: 4.0};
}
```

### In Enum Definitions

The `Result` is generic over two types, `T` and `E`, and has tow variants: `Ok`, which holds a value of type `T`, and `Err`, which holds a value of type `E`.

```rust
enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

> When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of the values they hold, you can avoid duplication by using generic types instead.

### In Method Definitions

```rust
struct Point<T> {
  x: T,
  y: T,
}
// implement a moethd named x on Point<T> to specify that we're implementing mehtods on the type Point<T>
impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

fn main(){
  let p = Point {x: 5, y: 10};

  println!("p.x = {}", p.x())
}
```

By declaring `T` as a generic type after `impl`, Rust can identify that the type in the angle brackets in `Point` is a generic type rather than a concrete type. Methods written within an `impl` that declares the generic type will be defined on any instance of the type, no matter what concrete type ends up substituting for the generic type.

```rust
// This code means the type Point<f32> will have a distance_from_origin method; other instances of Point<T> where T is not of type f32 will not have this method defined.
 impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

Generic type parameters in a struct definition aren't always the same as those you use in that same struct's method signatures.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // x=5, y=c
}
```

### Performance of code using generic

Using generic types won't make your program run any slower than it would with concrete types.

Rust accomplishes this by performing **monomorphization** of the code using generics at compile time. **Monomorphization** is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. In this process, the compiler does the opposite of the steps we used to create the generic function: the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

```rust
let integer = Some(5);
let float = Some(5.0);

// monomorphized version
enum Option_i32 {
  Some(i32),
  None,
}

enum Option_f64 {
  Some(f64),
  None,
}

fn main() {
  let integer = Option_i32::Some(5);
  let float = Option_f64::Some(5.0);
}
```

The generic `Option<T>` is replaced with the specific definitions created by the compiler. Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics. When the code runs, it performs just as it would if we had duplicated each definition by hand. **The process of monomorphization makes Rust’s generics extremely efficient at runtime.**

## Traits: Defining shared behavior

A **trait** defines functionality a particular type has and can share with other types. We can use **traits** to define shared behavior in an abstract way. We can use **trait bounds** to specify that a generic type can be any type that has certain behavior.

> Traits are similar to a feature often called **interfaces** in other languages, although with some differences.

### Define a trait

A type’s behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. **Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.**

```rust
pub trait Summary {
  fn summarize(&self) -> String;
}
```

Here, we declare a trait using the `trait` keyword and then the trait’s name, which is `Summary` in this case. We’ve also declared the trait as `pub` so that `crates` depending on this crate can make use of this trait too. Inside the curly brackets, we declare the `method signatures` that describe the behaviors of the types that implement this trait, which in this case is `fn summarize(&self) -> String.`

After the `method signature`, instead of providing an implementation within curly brackets, we use a **semicolon**. Each type implementing this `trait` must provide its own custom behavior for the body of the method. The compiler will enforce that any type that has the `Summary` trait will have the method `summarize` defined with this signature exactly.

**A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.**

### Implementing a trait on a type

Implementing a `trait` on a type is similar to implementing regular methods. The difference is that after `impl`, we put the `trait name` we want to implement, then use the `for` keyword, and then specify the name of the type we want to implement the trait for.

```rust
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

// impl `trait` for `the name of type`
impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}
```

Other `crates` that depend on the `aggregator` crate can also bring the `Summary` trait into scope to implement `Summary` on their own types. **One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate.** For example, we can implement standard library traits like `Display` on a custom type like `Tweet` as part of our `aggregator` crate functionality, because the type `Tweet` is local to our `aggregator` crate. We can also implement `Summary` on `Vec<T>` in our `aggregator` crate, because the `trait` `Summary` is local to our `aggregator` crate.

**But we can’t implement external traits on external types.** For example, we can’t implement the `Display` trait on `Vec<T>` within our aggregator crate, because `Display` and `Vec<T>` are both defined in the standard library and aren’t local to our `aggregator` crate. This restriction is part of a property called **coherence**, and more specifically **the orphan rule**, **so named because the parent type is not present**.

> This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two `crates` could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

```rust
use aggregator::{Summary, Tweet};

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of coruse, as you probably already know, people,"),
    reply: false,
    retweet: false,
  };
  println!("1 new tweet: {}", tweet.summarize());
}
```

### Default implementations

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}
// to use default implemention to summarize instances of NewsArticle
// we specify an empty impl block with ↓
impl Summary for NewsArticle {}

let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());


// New article available! (Read more...).
```

```rust
pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
      "of course, as you probably already know, people",
  ),
  reply: false,
  retweet: false
};

  println!("1 new tweet: {}", tweet.summarize());
}

// 1 new tweet: (Read more from @horse_ebooks...)
```

### traits as parameters

Use traits to define functions that accept many different types.

Instead of a concrete type for the `item` parameter, we specify the `impl` keyword and the **trait name.** This parameter accepts any type that implements the specified trait. In the body of `notify`, we can call any methods on item that come from the `Summary` trait, such as `summarize`. We can call `notify` and pass in any instance of `NewsArticle` or `Tweet`. **Code that calls the function with any other type, such as a `String` or an `i32`, won’t compile because those types don’t implement Summary.**

```rust
// item can be anything what implement Summary
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

// syntax sugar (trait bound)
pub fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}

// two parameters that implement Summary
pub fn notify(item1: &impl Summary, item2: &impl summary){}

// use `impl Trait`
pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// specifying multiple trait bounds with the + syntax
pub fn notify(item: &(impl Summary + Display)) {}

// + syntax with trait bounds on generic types
pub fn notify<T: Summary + Display>(item: T){}
```

### Clearer trait bounds with `where` clauses

Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function's name and its parameter list, making the function signature hard to read.

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(T: &T, u: &U) -> i32 {}

// use `where` clause
fn some_function<T, U>(t: &T, u: &U) -> i32 {
  where
    T: Display + Clone,
    U: Clone + Debug,
}
```

### Returning types that implement traits

By using `impl` `Summary` for the `return` type, we specify that the `returns_summarizable` function returns some type that implements the `Summary` trait without naming the concrete type. In this case, `returns_summarizable` returns a Tweet, but the code calling this function doesn’t need to know that.

**The ability to specify a return type only by the trait it implements is especially useful in the context of closures and iterators**. **Closures and iterators create types that only the compiler knows or types that are very long to specify.** The `impl Trait` syntax lets you concisely specify that a function returns some type that implements the Iterator trait without needing to write out a very long type.

```rust
//  impl Trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

However, you can only use `impl Trait` **if you’re returning a single type**. For example, this code that returns either a NewsArticle or a Tweet with the return type specified as impl Summary wouldn’t work:

Returning either a `NewsArticle` or a `Tweet` isn’t allowed due to restrictions around how the `impl Trait` syntax is implemented in the compiler.

```rust
// !!! wont work !!!!!
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

### Using trait bounds to conditionally implement methods

```rust
use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T,
}

// slways implements the `new` function to return a new instance of `Pair<T>`
impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self {x, y}
  }
}

// only implements the `cmp_display` method if
// its inner type `T` implements the `ParticalOrd` trait that enables comparision and
// the `Display` traint the enables printing
impl<T: Display + ParticalOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("the largest number is y = {}", self.y);
    }
  }
}
```

**We can also conditionally implement a trait for any type that implements another trait.** Implementations of a trait on any type that satisfies the trait bounds are called **blanket implementations** and are extensively used in the Rust standard library. For example, the standard library implements the `ToString` trait on any type that implements the `Display` trait.

```rust
impl<T: Display> ToString for T {}
```

Because the standard library has this blanket implementation, we can call the `to_string` method defined by the `ToString` trait on any type that implements the `Display` trait.

Blanket implementations appear in the documentation for the trait in the "implementors" section.

```rust
// turn integers into their corresponding String value
let s = 3.to_string();
```

`Traits` and `trait bounds` let us write code that uses `generic type parameters` **to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior.** The compiler can then use the `trait bound` information to check that all the concrete types used with our code provide the correct behavior. In dynamically typed languages, we would get an error at runtime if we called a method on a type which didn’t define the method. But **Rust moves these errors to compile time** so we’re forced to fix the problems before our code is even able to run. Additionally, we don’t have to write code that checks for behavior at runtime because we’ve already checked at compile time. Doing so improves performance without having to give up the flexibility of generics.

## Validating Reference with Lifetimes

Lifetimes are another kind of generic that we’ve already been using. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.

Every reference in Rust has a **lifetime**, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We only must annotate types when multiple types are possible. In a similar way, **we must annotate lifetimes when the lifetimes of references could be related in a few different ways.** Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

### Preventing dangling references with Lifetimes

The main aim of lifetimes is to prevent **dangling references**, which cause a program to reference data other than the data it's intended to reference.

```rust
fn main() {
  let r;

  {
    let x = 5;
    r = &x;  // ERROR: borrowed value does not live long enough
  }

  println!("r: {}", r);
}
```

The variable `x` doesn’t “live long enough.” The reason is that `x` will be out of scope when the inner scope ends on line 7. But r is still valid for the outer scope; because its scope is larger, we say that it “lives longer.” **If Rust allowed this code to work, `r` would be referencing memory that was deallocated when `x` went out of scope, and anything we tried to do with `r` wouldn’t work correctly**. So how does Rust determine that this code is invalid? It uses a borrow checker.

### Borrow checker

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

Here, `x` has the lifetime `'b`, which in this case is larger than `'a`. This means `r` can reference `x` because Rust knows that the reference in `r` will always be valid while `x` is valid.

```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          //   |       |
}                         // ----------+
```

### Generic lifetimes in functions

Note that we want the function to take **string slices**, which are **references**, rather than **strings**, because we don’t want the `longest` function to take ownership of its parameters.

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
//  The longest string is abcd
```

When we’re defining this function, we don’t know the concrete values that will be passed into this function, so we don’t know whether the if case or the else case will execute. We also don’t know the concrete lifetimes of the references that will be passed in, so we can’t look at the scopes as we did in Listings 10-17 and 10-18 to determine whether the reference we return will always be valid. The **borrow checker** can’t determine this either, because it doesn’t know how the **lifetimes** of `x` and `y` relate to the **lifetime** of the return value.

To fix this error, we’ll add **generic lifetime parameters** that define the relationship between the references so the **borrow checker** can perform its analysis.

```rust
// WONT WORK
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### Lifetine annotation syntax

Lifetime annotations don’t change how long any of the references live. Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes. Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

### Lifetime annotation in function signatures

The function signature now tells Rust that for some lifetime `'a`, the function takes two parameters, both of which are `string slices` that live at least as long as lifetime `'a`. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime `'a`. In practice, it means that the lifetime of the reference returned by the `longest` function is the same as the smaller of the lifetimes of the values referred to by the function arguments. These relationships are what we want Rust to use when analyzing this code.

Remember, **when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints.** Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

```rust
// WONT WORK
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### Thinking in terms of lifetimes

If we changed the implementation of the `longest` function to always return the first parameter rather than the longest string slice, we wouldn't need to specify a lifetime on the `y` parameter.

```rust
// WORK!
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters. If the reference returned does not refer to one of the parameters, it must refer to a value created within this function. However, this would be a dangling reference because the value will go out of scope at the end of the function. Consider this attempted implementation of the longest function that won’t compile:

```rust
// Fail because the return value lifetime isn't related to the lifetime of the parameters at all or its own type

// we can't return a reference to soemthing created within the function
// because once this function is over, the local variables get destroyed
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

// this works
fn longest<'a>(x: &str, y: &str) -> String {
  let r = String::from("really long string");
  r
}
```

The problem is that `result` goes out of scope and gets cleaned up at the end of the `longest` function. We’re also trying to return a reference to `result` from the function. There is no way we can specify lifetime parameters that would change **the dangling reference**, and Rust won’t let us create a dangling reference. **In this case, the best fix would be to return an owned data type rather than a reference so the calling function is then responsible for cleaning up the value.**

Ultimately, **lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.** Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

### Lifetime annotations in struct definitions

This `struct` has the single field `part` that holds a `string slice`, which is a `reference`. As with g`eneric data types`, we declare the name of the generic lifetime parameter inside angle brackets after the name of the struct so we can use the lifetime parameter in the body of the struct definition. This annotation means an instance of `ImportantExcerpt` can’t outlive the reference it holds in its `part` field.

The `main` function here creates an instance of the `ImportantExcerpt` struct that holds a reference to the first sentence of the `String` owned by the variable `novel`. The data in `novel` exists before the `ImportantExcerpt` instance is created. In addition, `novel` doesn’t go out of scope until after the `ImportantExcerpt` goes out of scope, so the reference in the `ImportantExcerpt` instance is valid.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention plaease: {}", announcement);
    self.part
  }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

### Lifetime Elision

The patterns programmed into Rust’s analysis of references are called **the lifetime elision rules**. These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

Lifetime on function or method parameters are called **input lifetimes** , and lifetimes on return values are called **output lifetimes**.

The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations. **The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes.** If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to `fn` definitions as well as `impl` blocks.

**The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.** In other words, **a function with one parameter gets one lifetime parameter**: `fn foo<'a>(x: &'a i32);` a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on.

**The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters**: `fn foo<'a>(x: &'a i32) -> &'a i32.`

**The third rule is that, if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters.** This third rule makes methods much nicer to read and write because fewer symbols are necessary.

```rust
fn first_word(s: &str) -> &str {}

// the compiler applies the firs rule, which specifies that each parameter gets its own lifetime.
fn first_word<'a>(s: &'a str) -> &str {}

// the second rule applies because there is exactly one onput lifetime
fn first_word<'a>(s: &'a str) -> &'a str {}
```

```rust
fn longest(x: &str, y: &str) -> &str {}

// apply the first rule: each parameter gets its own lifetime
fn logest<'a, 'b>(x: &'a str, y: &'b str) -> &str{}
```

You can see that the second rule doesn’t apply because there is more than one input lifetime. The third rule doesn’t apply either, because `longest` is a function rather than a method, so none of the parameters are self. After working through all three rules, we still haven’t figured out what the return type’s lifetime is. This is why we got an error trying to compile the code in Listing 10-20: the compiler worked through the lifetime elision rules but still couldn’t figure out all the lifetimes of the references in the signature.

Because the third rule really only applies in method signatures, we’ll look at lifetimes in that context next to see why the third rule means we don’t have to annotate lifetimes in method signatures very often.

### Lifetime annotations in method definitions

Lifetime names for `struct` fields always need to be declared after the `impl` keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.

```rust
impl<'a> ImportantExcerpt<'a> {
  fn level(&self) -> i32 {
    3
  }
}
```

The lifetime parameter declaration after `impl` and its use after the type name are required, but we’re not required to annotate the lifetime of the reference to self because of **the first elision rule**.

There are two input lifetimes, so Rust applies **the first lifetime elision rule** and gives both `&self` and `announcement` their own lifetimes. Then, because one of the parameters is `&self`, the return type gets the lifetime of `&self`, and all lifetimes have been accounted for.

```rust
// inclued lifetime manually
// Rule - 3
impl<'a> ImportantExcerpt<'a> {
  fn announce_and_return_part(&'a self, announcement: &str) -> &'a str {
    println!("Attention plaease: {}", announcement);
    self.part
  }
}
```

### The static lifetime

One special lifetime we need to discuss is `'static`, which denotes that the affected reference can live for the entire duration of the program.

The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is `'static`.

You might see suggestions to use the `'static` lifetime in error messages. But before specifying `'static` as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not, and whether you want it to. Most of the time, an error message suggesting the `'static` lifetime results from attempting to create a **dangling reference** or **a mismatch of the available lifetimes**. In such cases, the solution is fixing those problems, not specifying the `'static` lifetime.

```rust
let s: &'static str = "I have a static lifetime.";
```

## Generic type parameters, trait bounds, and lifetimes together

This is the `longest` function that returns the longer of two string slices. But now it has an extra parameter named ann of the generic type `T`, which can be filled in by any type that implements the `Display` trait as specified by the `where` clause. This extra parameter will be printed using {}, which is why `the Display trait bound` is necessary. Because lifetimes are a type of generic, the declarations of the lifetime parameter `'a` and the generic type parameter `T` go in the same list inside the angle brackets after the function name.

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}
```
