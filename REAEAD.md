#

## 01 rustc

```rs
fu main () {
  // a rust macro
  println!("Hello, world!");
}
```

- An ahead-of-time compiled language, meaning you can compile a program and dive the executable to someone else, and they can run it even without having Rust installed.

## 02 cargo

```sh
cargo new hello-cargo
cargo new --vcs=git hello-cargo # git is the default
cargo build # create an executable file in the target/debug/hello_cargo directory
cargo run # build and run a profect
cargo check # checks code to make sure it compiles but does not produce an executable
cargo build --release # build a release version of the project
```

```sh
git clone example.org/project
cd project
cargo build
```

- `cargo check` is much faster than `cargo build`, because it skips the step of producting an executable. Many Rustanceans run `cargo check` periodically as they write their program to make sure it compiles. Then they run `cargo build` when they're ready to use the executable.

- We can build a project using `cargo build`
- We can build and run a project in one step using `cargo run`
- We can build a project without producing a binary to check for errors using `cargo check`
- Cargo stores save the result of the build in the `targe/debug` directory
- `cargo build --release` to compile with optimaizations and create an executable in `target/rekease`
