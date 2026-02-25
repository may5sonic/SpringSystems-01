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