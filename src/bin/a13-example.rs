fn print_it(data: &str) {
    println!("{:?}", data)
}

fn main() {
    //  문자열 생성 시 borrowed 가 default
    print_it("a string slice");

    // owned 문자열 생성 2가지 방법
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");
    print_it(&owned_string);
    print_it(&another_owned);
}
