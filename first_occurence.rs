fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if (mid == 0 || arr[mid - 1] < target) && arr[mid] == target {
            return Some(mid);
        } else if target > arr[mid] {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}

fn main() {
    println!("Enter your array to check its occurrence or not: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input
       .trim()
       .split_whitespace()
       .map(|x| x.parse().expect("Please type numbers!"))
       .collect();

    println!("Enter the target number: ");
    let mut target_input = String::new();
    std::io::stdin().read_line(&mut target_input).expect("Failed to read line");
    let target: i32 = target_input.trim().parse().expect("Please type a number!");

    match find_first_occurrence(&arr, target) {
        Some(index) => println!("First occurrence of {} is at index {}", target, index),
        None => println!("Target not found in the array"),
    }
}