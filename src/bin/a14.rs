// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// enum FavoriteColor {
//     Red,
//     Blue,
//     Yellow,
// }

struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}

fn print_name(name: &str) {
    println!("name : {:?}", name);
}
fn print_color(color: &str) {
    println!("color : {:?}", color);
}

fn main() {
    let people = vec![
        Person {
            age: 20,
            name: "john".to_owned(),
            favorite_color: "blue".to_owned(),
        },
        Person {
            age: 10,
            name: "peter".to_owned(),
            favorite_color: "yellow".to_owned(),
        },
        Person {
            age: 7,
            name: "brown".to_owned(),
            favorite_color: "pink".to_owned(),
        },
    ];
    for person in &people {
        match person.age {
            1..=10 => {
                print_name(&person.name);
                print_color(&person.favorite_color);
            }
            _ => continue,
        }
    }
}
