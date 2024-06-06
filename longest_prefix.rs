fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = strs[0].clone();

    for s in &strs[1..] {
        let mut i = 0;
        while i < prefix.len() && i < s.len() && prefix.as_bytes()[i] == s.as_bytes()[i] {
            i += 1;
        }
        prefix.truncate(i);
        if prefix.is_empty() {
            break;
        }
    }

    prefix
}

fn main() {
    let strs = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let prefix = longest_common_prefix(strs);
    println!("The longest common prefix is: {}", prefix);
}
