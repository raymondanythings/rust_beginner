// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Brown,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
        };
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width = {:?}", self.width);
        println!("height = {:?}", self.height);
        println!("depth = {:?}", self.depth);
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            color,
            weight,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight = {:?}", self.weight)
    }
}

fn main() {
    let small_dimensions = Dimensions {
        depth: 10.0,
        height: 5.3,
        width: 11.6,
    };

    let large_dimensions = Dimensions {
        depth: 22.3,
        height: 30.1,
        width: 101.3,
    };

    let my_shipping_box = ShippingBox::new(21.3, Color::Brown, small_dimensions);
    my_shipping_box.print();
    let my_large_shipping_box = ShippingBox::new(101.3, Color::Red, large_dimensions);
    my_large_shipping_box.print();
}
