use std::io;

fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let rev: String = s.chars().rev().collect();
    s == rev
}

fn main() {
    println!("Enter your string to check if it's a palindrome or not:");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    if is_palindrome(input) {
        println!("'{}' is a palindrome.", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}