fn main() {
    let weight1 = 30;
    let weight2 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(weight1, weight2)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}
