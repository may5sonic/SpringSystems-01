use std::io::{self, Write};

fn most_frequent_word(text: &str) -> (String, usize) {

    // This splits text into words
    let words: Vec<&str> = text.split_whitespace().collect();

    // Create a vector to store tuple of (word, count)
    let mut word_counts: Vec<(&str, usize)> = Vec::new();

    //count the frequency of each of the words
    for word in words {
        let mut found = false;

        // Use a mutable reference (&mut) to modify the count if the word is already in the list
        for entry in &mut word_counts {
            if entry.0 == word {
                entry.1 += 1; // Modifying value via mutable reference
                found = true;
                break;
            }
        }

        // If we haven't seen the word yet. add it with a count of 1
        if !found {
            word_counts.push((word, 1));
        }
    }

    // Find the word with the highest frequency
    let mut max_word = "";
    let mut max_count = 0;

    for entry in word_counts {
        if entry.1 > max_count {
            max_count = entry.1;
            max_word = entry.0;
        }
    }

    // Return the tuple, converting the string slice to an owned String
    (max_word.to_string(), max_count)
    //(max_word, max_count) // return tuple
}

fn main() {
    println!("Select an option:");
    println!("A: Use default assignment text");
    println!("B: Input your own words");
    print!("> ");

    // Flush stdout to ensure the prompt "> " prints before waiting for input
    io::stdout().flush().unwrap();

    // Read the user's menu choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim().to_uppercase();

    // We must declare user_input here so it lives long enough for text_to_use to borrow it
    let mut user_input = String::new(); 
    
    // Determine which string to pass into our function
    let text_to_use = match choice.as_str() {
        "A" => {
            println!("\nUsing default text...");
            "the quick brown fox jumps over the lazy dog the quick brown fox"
        }
        "B" => {
            print!("\nEnter your text: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut user_input).expect("Failed to read line");
            user_input.trim() // Borrow the input as a string slice (&str)
        }
        _ => {
            println!("\nInvalid choice. Defaulting to Option A...");
            "the quick brown fox jumps over the lazy dog the quick brown fox"
        }
    };

    // Calculate and print the results
    let (word, count) = most_frequent_word(text_to_use);
    
    println!("---");
    if count == 0 {
        println!("No words were provided.");
    } else {
        println!("Most frequent word: \"{}\" ({} times)", word, count);
    }

    //let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    //let (word, count) = most_frequent_word(text);
    //println!("Most frequent word: \"{}\" ({} times)", word, count);
}
