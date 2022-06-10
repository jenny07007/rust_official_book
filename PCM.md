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
