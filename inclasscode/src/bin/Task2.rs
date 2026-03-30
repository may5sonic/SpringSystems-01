//let mut total = 0;
//let mut accumulate = || {
//    total += 5;
//    println!("Total: {}", total);
//};
//accumulate(); // Output: Total: 5
//accumulate(); // Output: Total: 10

fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        // Your implementation here
        tracker += 5;
        println!("Tracker: {}", tracker);
    };

    update();
    update();
}

fn main() {
    track_changes();
}