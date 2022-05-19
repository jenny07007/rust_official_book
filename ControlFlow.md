# Control Flow

```rs
// if expressions
fn main() {
  let number = 3;
  if (number != 0) {
    println!("{} is not zero", number);
  }
}

//  use if in a let statement
fn main() {
  let condition = true;
  let number = if condition {5} else {6};
  println!("The value of number is: {}", number); // 5
}

// loops with loops , breal & continue
fn main() {
  let mut count = 0;
  'counting_up': loop {
    println!("count = {}", count);
    let mut remainig = 10;

    loop {
      println!("remaining = {}", remaining)
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remainig -= 1;
    }
    count += 1;
  }
  println!("End count: {}", count); // 2
}

// return values from loops
fn main() {
  let mut count = 0;
  let result = loop {
    count += 1;
    if count == 10 {
      break count * 2;
    }
  };
  println!("The result is {}", result); // 20
}

// conditional loops with while
fn main() {
  let mut number = 3;
  while number != 0 {
    println!("{}!", number);
    number -= 1;
  }
  println!("LIFTOFF!!!");
}

// for loop with through a collection
fn main() {
  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

 while index < 5 {
   println!("the value is: {}", a[index]);
   index += 1;
 }

 for element in a {
    println!("the value is: {}", element);
 }

// countdown loop -- rev
  for number in (1..4).rev() {
    println!("{}!", number); // 3!, 2!, 1!,
  }
}
```
