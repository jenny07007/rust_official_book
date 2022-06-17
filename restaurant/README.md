#

- [Create a new library named `restaurant`](#create-a-new-library-named-restaurant)
- [Paths for referring to an item in the module tree](#paths-for-referring-to-an-item-in-the-module-tree)
- [Starting relative paths with `super`](#starting-relative-paths-with-super)
- [Making structs and enums public](#making-structs-and-enums-public)
- [Bring paths into scope with the `use` keyword](#bring-paths-into-scope-with-the-use-keyword)
- [Re-exporting names with public `use`](#re-exporting-names-with-public-use)
- [Using external packages](#using-external-packages)
- [Using nested paths to clean up large use lists](#using-nested-paths-to-clean-up-large-use-lists)
- [The glob operator](#the-glob-operator)
- [Separating modules into different files](#separating-modules-into-different-files)

## Create a new library named `restaurant`

```bash
cargo new --lib restaurant
```

In the restaurant industary, some parts of restaurant are referred to as **front of house** and others as **back of house**. Front of house is where customers are; this is wher hosts seat customers, servers take orders and payments, and bartenders make drinks. Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do a adminstrative work.

[src/lib.rs](./src/lib.rs)

By using modules, we can group related definitions together and name why they are related. Programmers using this code would have an easier time finding the definitions they wanted to use because they would navigate the code on the groups rather than having to read through all the definitions. Programmers adding new functionality to this code would know where to place the code to keep the program organized.

`src/main.rs` and `src/lib.rs` are called crate roots, it is because the contents of either of these two files form a module named `crate` at the root of the crate's module structure, known as the **module tree**.

```bash
# module tree
crate
|__ front_of_house
    |__ hosting
    |   |__ add_to_waitlist
    |   |__ seat_at_table
    |__ serving
        |__ take_order
        |__ serve_order
        |__ take_payment
```

Just like directories in a filesystem, you use modules to organize your code. And just like files in a directory, we need a way to find our modules.

## Paths for referring to an item in the module tree

A path can take two forms:

- An **absolute path** starts from a **crate root** by using a crate name (for code from an external crate) or a literal `crate` (for code from the current crate).
- A **relative path** starts from the current module and uses `self`, `super`, or an identifier in the current module.

```rs
// src/lib.rs
// won't compile!!!!!!!! private modules!@!@!@!
mod front_of_house{
    // change to pub
    pub mod hosting{
        // here also change to pub
        pub fn add_to_waitlist(){}
    }
}
// maked a `pub` keyword because it is part of our library crate's public API
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

Best practices for packages with a Binary and a Library

A package can contain both a `src/main.rs` binary crate root and a `src/lib.rs` library crate root, and both crates will have the package name by default. Typically, packages with this pattern will have just enough code in the binary crate to start an executable that calls code with the library crate. This lets other projects benefit from the most functionality that the package provides, because the library crate's code can be shared.

The module tree should be defined in `src/lib.rs`. Then, any piblic items can be used in the binary crate by starting paths with the name of the package. The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: it can use the public API.

## Starting relative paths with `super`

The `fix_incorrect_order` function is in the `back_of_house` module, so we can use `super` to go to the parent module of `back_of_house`, which in this case is `crate`, the root. From there, we look for `deliver_order` and find it.

We think the `back_of_house` module and the `deliver_order` function are likely to stay in the same relationship to each other and get moved together should we decide to reorganize the crate's module tree. Therefore, we use `super` so we'll have fewer places to update code in the future if this code gets moved to a different module.

```rs
// src/lib.rs
fn deliver_order(){}

mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        // super -> is like starting a filesystem path with the `..` syntax
        super::deliver_order();
    }
    fn cook_order(){}
}
```

## Making structs and enums public

We can use `pub` to designate structs and enums as public, but there are a few extra details.If we use `pub` before a struct defination, we make the struct public, but the struct's fields will still private. We can make each field public or on a case-by-case basis.

```rs
// src/lib.rs
mod back_of_house{
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String, // private
    }
    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("apple"),  // private
            }
        }
    }
}

pub fn eat_at_restaurant(){
    // order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it;
    // we're not allowed see or modify the seasonal fruit that comes with the meal
    meal.seasonal_fruit = String::from("blueberries");
}
```

If we make an enum public, all of its variants are then public.

```rs
mod back_of_house{
    pub enum Appetizer{
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant(){
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

Enums are not very useful useless their variants are public; the default for enum variants is to be public.

Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private be default unless annotated with `pub`.

## Bring paths into scope with the `use` keyword

Bringing the function's parent module into scope with `use` means we have to specify the parent module when calling the function. Specifying the parent module when calling the function makes it clear that the function is not locally defined while still minimizing repetition of the full path.

```rs
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;
pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```

When bringing in structs, enums, and other items with `use`, it's idiomatic to specify the full path.

```rs
use std::collection::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

Exception if we're bringing two items with the same name into scope with `use` statements, because Rust does not allow that.

```rs
// use parent modules distinguishes the two Result types.
use std::fmt;
use std::io;
// or provide new names with as keyword
use std::io::Result as IoResult;


fn function1() -> fmt::Result {}
fn function2() -> io::Result<()> {}
fn function3() -> IoResult<()> {}
```

## Re-exporting names with public `use`

When we bring a name into scope with `use`, the name available in the new scope is private.To enable the code that calls our code to refer to that name as if it had been defined in thhat code's scope, we can use the `pub use` keyword. This technique is called re-exporting because we're bringing an item into scope but also making that item available for others to bring into their scope.

Re-exporting is useful when the internal structure of your code is different from how pargrammers calling your code would think about the domain.

```rs
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// reexporting the hosting module from root module
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

## Using external packages

```toml
rand = "0.8.3"
```

```rs
use rand::Rng;
// std a crate that's external to our package.
// the standard library is shipped with the Rust languate, we don't need to change Cargo.toml
use std::collection::HashMap;

fn main() {
    let screte_number = rand::thread_rng().gen_range(1..101);
}
```

## Using nested paths to clean up large use lists

```rs
// main.rs
use std::cmp::Ordering;
use std::io::Write;
use std::io;
// instend
use std::{cmp::Ordering, io};
use std::io::{self, Write};
```

## The glob operator

The glob operator is often used when testing to bring everything test into the `tests` module.

```rs
use std::collections::*;
```

## Separating modules into different files

```rs
// src/lib.rs
mod front_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

If we put `hosting.rs` in the src directory, the compiler would expect that code to be in a `hosting` module declared in the crate root, not as a child of the `front_of_house` module. The rules the compiler follows to know what files to look in for modules' code means the dorectories and files more closely match the module tree.

```rs
// src/front_of_house.rs
pub mod hosting;
```

```rs
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```
