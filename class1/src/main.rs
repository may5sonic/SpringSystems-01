/*

use std::arch::asm;

fn main() {
    let message = b"Hello, direct syscall!\n";

    unsafe {
        // write syscall
        asm!(
            "mov rax, 1",  // syscall number for write
            "mov rdi, 1",  // file descriptor: 1 is stdout
            "syscall",
            in("rsi") message.as_ptr(),
            in("rdx") message.len(),
            out("rax") _,
            out("rcx") _,
            out("r11") _,
            clobber_abi("system")
        );

        // exit syscall
        asm!(
            "mov rax, 60", // syscall number for exit
            "xor rdi, rdi", // status code 0
            "syscall",
            options(noreturn)
        );
    }
}
*/

/*
use std::fs::File;
use std::io::{Read, BufReader, BufRead};

fn read_entire_file() {
    let mut file = File::open("example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents:\n{}", contents);
}

fn read_file_line_by_line() {
    let file = File::open("example.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn main() {
    println!("Reading entire file:");
    read_entire_file();

    println!("\nReading file line by line:");
    read_file_line_by_line();
}
*/


    /*
    use std::fs::File;
use std::io::Write;

fn create_and_write_to_file() {
    let mut file = File::create("example.txt").unwrap();
    writeln!(file, "Hello, Rust file operations!").unwrap();
    writeln!(file, "This is a new line.").unwrap();
}

fn main() {
    create_and_write_to_file();
    println!("File created and written successfully.");
}
*/

/*
use std::fs::OpenOptions;
use std::io::Write;

fn append_to_file() {
    let mut file = OpenOptions::new()
        .append(true)
        .open("example.txt")
        .unwrap();

    writeln!(file, "This line is appended to the file.").unwrap();
}

fn main() {
    append_to_file();
    println!("Successfully appended to the file.");
}
*/

/*
use std::path::{Path, PathBuf};

fn work_with_paths() {
    let path = Path::new("example.txt"); // immutable
    println!("File name: {:?}", path.file_name());
    println!("Parent directory: {:?}", path.parent());

    let mut path_buf = PathBuf::from("/tmp"); // mutable
    path_buf.push("rust_files");
    path_buf.push("example.txt");
    println!("Full path: {:?}", path_buf);
}

fn main() {
    work_with_paths();
}
*/

/*
use std::fs::{self, File};
use std::io::Write;

fn advanced_file_operations() {
    // Create a directory
    fs::create_dir_all("./test_dir").unwrap();

    // Create a file and write to it
    let mut file = File::create("./test_dir/test.txt").unwrap();
    writeln!(file, "Hello, advanced operations!").unwrap();

    // Get file metadata
    let metadata = file.metadata().unwrap();
    println!("File size: {} bytes", metadata.len());
    println!("Is directory: {}", metadata.is_dir());

    // List directory contents
    let entries = fs::read_dir("./test_dir").unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        println!("Found entry: {:?}", entry.path());
    }

    // Remove file and directory
    fs::remove_file("./test_dir/test.txt").unwrap();
    fs::remove_dir("./test_dir").unwrap();
}

fn main() {
    advanced_file_operations();
}
    */


/*
use std::process::Command;

fn executing_os_commands_linux() {
    let output = Command::new("ls")
        .arg("-l")
        .output()
        .expect("Failed to execute command");

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    executing_os_commands_linux()
}
*/


/*
use std::fs::File;
use std::io::Write;
use std::process::Command;

struct LogSummary {
    info_count: u32,
    warn_count: u32,
    error_count: u32,
}

impl LogSummary {
    fn new() -> LogSummary {
        LogSummary {
            info_count: 0,
            warn_count: 0,
            error_count: 0,
        }
    }

    fn process_log(&mut self, log: &str) {
        if log.contains("INFO") {
            self.info_count += 1;
        } else if log.contains("WARN") {
            self.warn_count += 1;
        } else if log.contains("ERROR") {
            self.error_count += 1;
        }
    }

    fn save_to_file(&self) {
        let mut file = File::create("log_summary.txt").unwrap();
        writeln!(file, "INFO: {}", self.info_count).unwrap();
        writeln!(file, "WARN: {}", self.warn_count).unwrap();
        writeln!(file, "ERROR: {}", self.error_count).unwrap();
    }

    fn execute_python_script(&self) {
        Command::new("python")
            .arg("generate_dashboard.py")
            .status()
            .unwrap();
    }
}

fn main() {
    let logs = [
        "INFO: Operation successful",
        "ERROR: Failed to connect",
        "WARN: Low battery",
        "INFO: Data synced",
        "ERROR: Timeout occurred",
    ];

    let mut summary = LogSummary::new();
    for log in logs.iter() {
        summary.process_log(log);
    }
    summary.save_to_file();
    summary.execute_python_script();
}
*/

/*
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let xs: Vec<i32> = new::Vec();
    let ys: Vec<i32> = new::Vec();

    (xs[index], ys[index])

    let point: Vec<Point> = new::Vec()
}

*/

/*
use std::io::{self, Read, Write};

struct Person {
    name: String,
    age: u32,
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("How old are you? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let age = buffer.trim().parse().unwrap();

    let person = Person { name, age };
    println!("Hi {}, you are {} years old!", person.name, person.age);
}
    

use std::fs::File;
use std::io::prelude::*;

struct Config {
    username: String,
    api_key: String,
    port: u16,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let username = lines.next().unwrap().to_string();
        let api_key = lines.next().unwrap().to_string();
        let port = lines.next().unwrap().parse().unwrap();

        Config { username, api_key, port }
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("username: {}", config.username);
    println!("api key: {}", config.api_key);
    println!("port: {}", config.port);
}

fn main() {
    reading_from_console();
    reading_from_file();
}
*/