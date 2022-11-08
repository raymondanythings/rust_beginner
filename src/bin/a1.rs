// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
fn get_input_typing(prompt_text: &str) -> String {
    let mut return_value = String::new();
    println!("{} : ", prompt_text);
    std::io::stdin().read_line(&mut return_value).unwrap();
    return_value
}
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    let first_name = get_input_typing("Enter Your First Name");
    let last_name = get_input_typing("Enter Your Last Name");
    println!("Hello!, {} {}", last_name, first_name);
}
