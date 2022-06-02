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

## Methods syntax

`Methods` are similar to functions: we declare them with the `fn` keyword and a name, they can have parameters and return values, and they contain some code that's run when the method is called from somewhere else.
Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always `self`, which represents the instance of the struct or enum that the method is being called on.

```rs
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

  // implementation
  // will be associated with the `Rectangle` type
  impl Rectangle {
    // self: &self
    fn area(&self) -> u32 {
      self.width * self.height
    }
  }

  println!(
      "The area of the rectangle is {} square pixels.",
      rect1.area()
  );
}
```

Methods like this are called `getters`, and Rust does not implement them automatically for struct fields as some other languages do.

Getters are useful because you can make the field private but the method public and thus enable read-only access to that field as part of the type's public API.

```rs
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

## `->` operator

In C and C++, two different operators are used for calling methods: you use `.` If you're calling a method on the object directly and `->` if you're calling the method on a pointer to the object and need to dereference the pointer first. In other words, if `object` is a pointer, `object -> something` is similar to `(*object).something`

Rust does not have a `->` operator, instead, Rust has a feature called **automatic referencing and dereferencing**.

When you call a method with `object.something()`, Rust automatically adds in `&`, `&mut`, or `*` so `object` matches the signature of the method.

```rs
// they are the same
pl.distance(&p2);
(&pl).distance(&p2);
```

## Methods with more parameters

```rs
// getter
impl Rectangle {
    fn area(&self) -> bool {
        self.width > 0
    }
// takes an immutable borrow of another `Rectangle` as a parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

## Associated functions

All functions defined with an `impl` block are called **associated functions**. We can define associated functions that don't have `self` as their first parameter (and thus are not methods) because they don't need an instance of the type to work with. We've already used one function like `String::from` function that is defined on the `String` type.

Associated functions that are not methods are often used for constructors that will return a new instance of the struct.

```rs
impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

// namespaced by the struct:
// the `::` syntax is used for both associated functions and namespacs created by modules
let sq = Rectangle::square(3);
```

## Multiple impl Blocks

Each `struct` is allowed to have multiple `impl` block.

```rs
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```
