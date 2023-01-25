# Data Types

## Scalar types -- intergers, floating-point numbers, booleans, characters

1. Scalar types
   Rust is a `statically typed language`, which means that it must know the types of all variables at compile time.

2. Integer types

   'signed' & 'unsigned' refer to whether it's possible for the number to be negative. whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned).

   - `u8` = 0 to 2\*\*8 - 1 (255)
   - `i8` = -(2**(8-1)) to 2**(8 - 1) - 1 = -128 to 127

   - `isize` and `usize` types depned on the architecture of the computer your program is running on, which is denoted in the tables as `arch`.

   | Length  | Signed | Unsigned |
   | :-----: | :----: | :------: |
   |  8-bit  |   i8   |    u8    |
   | 16-bit  |  i16   |   u16    |
   | 32-bit  |  i32   |   u32    |
   | 64-bit  |  i64   |   u32    |
   | 128-bit |  i128  |   u128   |
   |  arch   | isize  |  usize   |

   - integer literals

   | Number literals | Examples    |
   | --------------- | ----------- |
   | Decimal         | 98_222      |
   | Hex             | oxff        |
   | Octal           | 0o77        |
   | Binary          | 0b1111_0000 |
   | Byte (u8)       | b'A'        |

- integer types default to `i32`. the primary situation in which you'd use `isize` or `usize` is when indexing some sort of collection.
- `integer overflow` - a value is outside a type range, ex u8 is 0 to 255, but 256 is outside the range.
- `panic` - when a program exists with an error
- To handle the possibility of overflow:
  - wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`
  - return the `None` value if there is overflow with the `checked_*` methods
  - return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods
  - saturate at the value's minimum or maximum values with the `saturating_*` methods

3. Floating-Point Types

`f32` and `f64` are the floating-point types in Rust. The `f32` type is a single-percision float, and `f64` has double precision.

```rust
fn main() {
  let x = 2.0; // f64
  let y: f32 = 3.0; // f32
}
```

4. Numeric Operations

5. The boolean type

6. The character type
   `char` type is four bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII.

```rust
fn main( ) {
  let c = 'z';
  let z = 'ℤ';
  let heart_eyed_cat = '😻';
}
```

## compound types

can group multiple values into one type

1. The tuple type

- has a fixed length: once declared, they cannot grow or shrink in size.

```rust
fn main() {
  // tup - a fixed size array of related data that could be a different type
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x,y,z) = (500, 6.4, 1);
  println!("The value of y is: {}", y); // 6.4

// destructuring tup
  let (x:i32, y:f64, z:u8) = tup;
// dot notation tup
  let first_x:i32 = tup.1;

  let x: (i32, f64, u8) = (500, 6.4, 1);
  let five_hundred = x.0;
  let six_point_four = x.1;
  let one = x.2;
}
```

2. The array type

- must have the same type

```rust
fn mian() {
  let a = [1,2,3,4,5];
  let a: [i32; 5] = [1,2,3,4,5];
  let a = [3; 5]; // [3, 3, 3, 3, 3]

  let first = a[0];
  let second = a[1];
}
```

```rust
fn main() {
  let error_codes = [200, 404, 500];
  let not_found = error_codes[1];
}
```
