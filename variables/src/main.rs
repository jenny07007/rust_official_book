fn main() {
    let x = 5;
    let x = x + 1;
    // the first variable is 'shadowed' by the second,
    // the second variable's value is what the program sees when the variable is used
    {
        let x = x * 2;
        println!("The value of x in the inner scope us: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("{}", spaces);
}
