// fn process_vector_with_map<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
//     F: Fn(i32) -> i32,
// {
//     vec.into_iter().map(f).collect()
// }

// fn main() {
//     let numbers = vec![1, 2, 3];
//     let doubled = process_vector_with_map(numbers.clone(), |x| { /* your transform */ });
//     let replaced = process_vector_with_map(numbers, |x| { /* your transform */ });

//     println!("Doubled: {:?}", doubled);
//     println!("Replaced: {:?}", replaced);
// }

// fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
//     F: Fn(i32) -> i32,
// {
//     let mut result = Vec::new();
//     for x in vec {
//         result.push(f(x)); // Apply the closure
//     }
//     result
// }

fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // Your implementation here
    // I think I just create a vec and then iterate to each one in the vectoer so I can map it and collect it
    vec.into_iter().map(f).collect()
}

fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        // Implement: multiply each number by 2
          x * 2
    });

    let replaced = process_vector(numbers, |x| {
        // Implement: if number > 2, replace with 0, else keep number
        if x > 2 {
            // this is the part where it replaces the value to zero
            0
        } else {
            // This is the part that has the numbers the same
            x
        }
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}