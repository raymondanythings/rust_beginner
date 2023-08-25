fn main() {
    let mut i = 3;
    loop {
        println!("{:?}", i);
        i -= 1;
        if i == 0 {
            break;
        }
    }
    println!("loop DONE");
    i = 1;
    while i <= 3 {
        println!("{:?}", i);
        i += 1;
    }
    println!("while DONE");
}
