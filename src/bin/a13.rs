// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

struct Test {
    score: i32,
}

fn main() {
    let my_scores = vec![
        Test { score: 10 },
        Test { score: 20 },
        Test { score: 30 },
        Test { score: 40 },
    ];

    let mut total = 0;
    for test in &my_scores {
        total += test.score;
        println!("score = {:?}", test.score);
    }
    println!(
        "total value = {:?}, score length = {:?}",
        total,
        my_scores.len()
    )
}
