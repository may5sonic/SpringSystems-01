fn main() {
    let secret = 42;

    let mut guess_count = 0;

    loop {
        let mut guess = 39; // + guess_count;

        guess += guess_count;

        guess_count += 1;
        
        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess {}: {} is Correct!", guess_count, guess);
            break;
        } else if result == 1 { 
            println!("Guess {}: {} is too high.", guess_count, guess);
        } else {
            println!("Guess {}: {} is too low.", guess_count, guess);
        }
    }
    println!("It took {} guesses to find the number.", guess_count);
}

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}
