//let add = |x: i32, y: i32| x + y;
//println!("5 + 3 = {}", add(5, 3)); // Output: 5 + 3 = 8

fn main() {
    let operation = |a: i32, b: i32| {
        // Your implementation here
        // closure with explicit type (was the best one that worked for this)
        a + b
    };

    println!("Result: {}", operation(10, 5));
}