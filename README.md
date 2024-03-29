#

- [](#)
  - [03 Guessing game](#03-guessing-game)
  - [04 Variables](#04-variables)
  - [05 Data Types](#05-data-types)
  - [06 Functions](#06-functions)
  - [07 Control Flow](#07-control-flow)
  - [08 Ownership](#08-ownership)
  - [09 Strucs](#09-strucs)
  - [10 Enums and pattern matching](#10-enums-and-pattern-matching)
  - [11 Packages, Crates, and Modules](#11-packages-crates-and-modules)
  - [12 Common Collections](#12-common-collections)
  - [13 Error Handling](#13-error-handling)
  - [14 Generic Types, Traits, and Lifetimes](#14-generic-types-traits-and-lifetimes)
  - [15 Automated Tests](#15-automated-tests)
  - [16 Building a command line program](#16-building-a-command-line-program)

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

## 03 Guessing game

[Guessing game](./Guessing_game.md)

## 04 Variables

[Variables](./Variables.md)

## 05 Data Types

[DataTypes](./DataTypes.md)

## 06 Functions

[Functions](./Functions.md)

## 07 Control Flow

[ControlFlow](./ControlFlow.md)

## 08 Ownership

[OwnerShip](./OwnerShip.md)

## 09 Strucs

[Structs](./Struct.md)

## 10 Enums and pattern matching

[Enums](./Enums.md)

## 11 Packages, Crates, and Modules

[PCM](./PCM.md)

## 12 Common Collections

[Collections](./Collections.md)

## 13 Error Handling

[Error handling](./ErrorHandling.md)

## 14 Generic Types, Traits, and Lifetimes

[GTL](./GTL.md)

## 15 Automated Tests

[Tests](./Tests.md)

## 16 Building a command line program

[minigrep](./Minigrep.md)
