fn main() {
    let user_name = "tom";

    match user_name {
        "john" | "tom" | "emy" => println!("Hello {}", user_name),
        _ => println!("Please regist first."),
    }
}
