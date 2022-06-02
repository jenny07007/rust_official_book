#

## Refactoring with Tuples

```rs
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
}
```

## Refactoring with Structs

```rs
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}
```

## Adding useful functionality with derived traits

```rs
// the `Debug` trait enables us to print the struct in a human readable format
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // `Rectangle` does not implement `std::fmt::Display`
    // use `:?` to tell println to use `Debug` instead
    println!("rect1 is {}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}
```

Another way to print out a value using the `Debug` format is to use the `dbg!` macro, which takes ownership of an expression, prints the file and line number where that `deg!` macro call occurs in your code along with the resulting value of that expressionm and returns ownership of the value.

> Calling the `dbg!` macro prints to the standard error console stream `(stderr)`, as opposed to `println!` which prints to the standard output stream `(stdout)`.

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
      // can do this, because `deg!` returns ownership of the expression's value
      // the  `width` field will get the same value as if we didn't have the `dbg!` call there
        width: dbg!(30 * scale),
        height: 50,
    };

  // we don't want to `dbg!` to take ownership of `rect1`, so we use a reference to `rect1`
    dbg!(&rect1);
}
```
