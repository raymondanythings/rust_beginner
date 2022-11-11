struct LineItem {
    name: String,
    count: i32,
}
fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem {
            name: String::from("fruit"),
            count: 3,
        },
    ];

    for item in &receipt {
        println!("name : {:?}", item.name);
        println!("count : {:?}", item.count);
        println!()
    }
    println!("total length : {:?}", receipt.len());
}
