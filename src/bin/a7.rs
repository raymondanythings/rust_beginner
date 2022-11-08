// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Red,
    Blue,
    Yellow = 3,
}

fn get_color(color: Colors) -> i32 {
    match color {
        Colors::Red => 1,
        Colors::Blue => 2,
        Colors::Yellow => 3,
    }
}
fn main() {
    let color = get_color(Colors::Blue);
    println!("{:?}", color);
}
