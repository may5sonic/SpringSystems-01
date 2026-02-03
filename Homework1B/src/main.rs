fn main() {
    let numbers = [1, 2, 3, 4, 5, 15, 18, 20, 30, 7];

    println!("--- Analyzing Numbers ---");
    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        }
        else if num % 3 == 0 {
            println!("{}: Fizz", num);
        }
        else if num % 5 == 0 {
            println!("{}: Buzz", num);
        }
        else {
            if is_even(num) {
                println!("{}: Even", num);
            } else {
                println!("{}: Odd", num);
            }
        }
    }

    println!("\n--- Sum Calculation ---");

    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }

    println!("The sum of all numbers is: {}", sum);
    println!("\n--- Finding Largest Number ---");
    let mut largest = numbers[0];
    let mut i = 1;

    loop {
       if i >= numbers.len() {
        break;
       } 
       if numbers[i] > largest {
        largest = numbers[i];
       }
       i += 1;
    }
    println!("The largest number is: {}", largest);
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}
