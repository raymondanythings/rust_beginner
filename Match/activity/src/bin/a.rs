// Topic : Decision making with math

// Program requirements:
//     * Display "it's true" or "it's false" based on the value
//       a boolean variable

// Notes:
//     * Ise a varoab;e set tp eotjer true or false
//     * Use a match expression to determine which message to display

fn main() {
    let var = true;
    match var {
        true => println!("it's true"),
        false => println!("it's false"),
    }
}
