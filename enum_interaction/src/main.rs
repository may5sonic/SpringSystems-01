use std::io::{self, Write};
use std::process::Command;

// Define the FileOperation Enum
enum FileOperation {
    List(String),           // Directory path
    Display(String),        // File path
    Create(String, String), // File path and content
    Remove(String),         // File path
    Pwd,                    // Print working directory
}

// Implement the perform_operation Function
fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            if let Err(_) = Command::new("ls").arg(&path).status() {
                eprintln!("Failed to execute ls command.");
            }
        }
        FileOperation::Display(path) => {
            if let Err(_) = Command::new("cat").arg(&path).status() {
                eprintln!("Failed to execute cat command.");
            }
        }
        FileOperation::Create(path, content) => {
            let command_str = format!("echo '{}' > {}", content, path);
            let output = Command::new("sh")
            .arg("-c")
            .arg(&command_str)
            .output();

            match output {
                Ok(out) if out.status.success() => {
                    println!("\nFile '{}' created successfully.", path);
                }
                _ => {
                    eprintln!("Failed to create file.");
                }
            }
        }
        FileOperation::Remove(path) => {
            let status = Command::new("rm").arg(&path).status();
            match status {
                Ok(stat) if stat.success() => {
                    println!("\nFile '{}' removed successfully.", path);
                }
                _ => {
                    eprintln!("Failed to remove file.");
                }
            }
        }
        FileOperation::Pwd => {
            if let Err(_) = Command::new("pwd").status() {
                eprintln!("Failed to execute pwd command.");
            }
        }
    }
}

// Helper function to easily read user input
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before taking input

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        let choice = read_input("\nEnter your choice (0-5): ");

        match choice.as_str() {
            "1" => {
                let path = read_input("Enter directory path: ");
                perform_operation(FileOperation::List(path));
            }
            "2" => {
                let path = read_input("Enter file path: ");
                perform_operation(FileOperation::Display(path));
            }
            "3" => {
                let path = read_input("Enter file path: ");
                let content = read_input("Enter content: ");
                perform_operation(FileOperation::Create(path, content));
            }
            "4" => {
                let path = read_input("Enter file path: ");
                perform_operation(FileOperation::Remove(path));
            }
            "5" => {
                perform_operation(FileOperation::Pwd);
            }
            "0" => {
                println!("\nGoodbye!");
                break;
                // Program Termination
            }
            _ => {
                // Minimal Error Handling for menu selection
                println!("\nInvalid menu selection. Please enter a number between 0 and 5.");
            }
        }
    }
}
