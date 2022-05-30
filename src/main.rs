
use std::io;

fn main() {
    println!("Palindrome Checker");

    println!("Enter a word: ");
    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");


}
