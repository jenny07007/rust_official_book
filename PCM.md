#

## Managine growing projects with packages, crates, and modules

- Packages: A Cargo feature that lets you build, test, and share Rust code.
- Crate: A tree of modules that produces a library or executable.
- Modules and use: Let you control the organization, scope, and privacy of paths.
- Paths: A way of naming an item, such as a struct, function, or module.

## Packages and crates

A **package** is one or more crates that provide a set of functionality. A package contains a `Cargo.toml` file that describes how to build those crates.

A **crate** can be a binary crate or a library crate. **Binary crates** are programs you can complie to an executable that you can run, such as a command-line program or a server. They must have a function called `main` that defines what happens when the executable runs.

**Library crates** don't have a `main` function, and they don't compile to an executable. They define functionality intended to be shared with multiple projects. For example, the `rand` crate that generates random numbers.

The **crate root** is a source file that the Rust compiler starts from and makes up the root module of your crate.

A package can contain at most one library crate. It can contain as many binary crates as you'd like, but it must contain at least one crate (either library or binary).

```bash
cargo new my-project # Crated binary (application) `my-project` package
ls my-project # Cargo.toml src/
ls my-project/src # main.rs
```

## Defining modules to control scope and privacy

- `paths`: allow you to name items
- `use`: brings a path into scope
- `pub`: makes items public
- `as`
- external packages
- `glob`

---

- **Start from the crate root**: When compiling a crate, the compiler first looks in the crate root file (usually `src/lib.rs` for a library create or `src/main.rs` for a binary crate).
- **Declaring modules**: In the crate root file, you can declare a module named, say, "garden"m with`mod garden;`. The compiler will look for the code inside the module in these places:
  - inline, directly following `mod garden`, within curely brackets instend of the semicolon
  - in the file `src/garden.rs`
  - in the file `src/garden/mod.rs`
- **Declaring submodules**: In any file other than the crate root that's being compiled as part of the crate(for example, `src/garden.rs`), you can declare submodules(for example, `mod vegetables;`). The compiler will look for the code inside submodules in these places within a directory named for the parent module:
  - inline, directly following `mod vegetables`, within curely brackets instend of the semicolon
  - in the file `src/garden/vegetables.rs`
  - in the file `src/garden/vegetables/mod.rs`
- **Paths to code in modules**: Once a module is being compiled as part of your crate, you can refer to code in that module (for example, an `asparagus` type in the garden vegetables module) from anywhere else in this crate by using the path `crate::garden::vegetables::asparagus` as long as the privacy rules allow.
- **Privacy vs public**: Code within a module is private from its parent modules by default. To make a module public, delcare it with `pub mod` instend of `mod`. To make items within a public module public as well, use `pub` before their declarations.
- **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;`. and then only need to write `Asparagus` to make use of that type in the scope.

```bash
backyard
|__ Cargo.local
|__ Cargo.toml
|__ src
    |__ garden
    |   |__ vegetables.rs
    |__ garden.rs
    |__ main.rs
```

```rs
// src/main.rs
use crate::garden::vegetables::Asparagus;

// means the compiler includes the code it finds in src/garden.rs
// which is src/garden.rs
pub mod garden;

fn main() {
  let plant = Asparagus {};
  println!("I'm growing {:?}", plant);
}
```

```rs
// src/garden.rs
// means the code in src/garden/vegetables.rs
pub mod vegetables;
```

```rs
// src/garden/vegetables.rs
pub struct Asparagus {}
```

## Grouping related code in modules

**Modules** let us organize code within a **crate** into groups for readability and easy reuse. Modules also control the **privacy** of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).

[Example - restaurant library crate](./restaurant/README.md)
