fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {

    let a = add(1,1);
    let b = add(2,3);
    let c = add(a,2);
    println!("{}",c);
    println!("Hello, world!");
}
