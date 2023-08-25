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

enum Variants {
    Red,
    Blue,
    Orange,
    Purple,
}

fn print_color_name(variants: Variants) {
    match variants {
        Variants::Red => println!("color name is Red"),
        Variants::Blue => println!("color name is Blue"),
        Variants::Orange => println!("color name is Orange"),
        Variants::Purple => println!("color name is Purple"),
    }
}

fn main() {
    let color = Variants::Red;
    print_color_name(color);
    let color = Variants::Blue;
    print_color_name(color);
    let color = Variants::Orange;
    print_color_name(color);
    let color = Variants::Purple;
    print_color_name(color);
}
