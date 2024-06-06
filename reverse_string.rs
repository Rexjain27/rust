fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let input = "rishabh bothra";
    let reversed = reverse_string(input);
    println!("The reversed string is: {}", reversed);
}