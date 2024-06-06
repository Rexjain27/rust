fn shortest_word(s: &str) -> &str {
    let words: Vec<_> = s.split_whitespace().collect();
    let shortest = words.iter().min_by_key(|w| w.len()).unwrap();
    shortest
}

fn main() {
    let input = "This is a Internship test.";
    let shortest = shortest_word(input);
    println!("The shortest word is: {}", shortest);
}