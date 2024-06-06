fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];
    for &x in &arr[1..] {
        current_sum = std::cmp::max(x, current_sum + x);
        max_sum = std::cmp::max(max_sum, current_sum);
    }
    max_sum
}

fn main() {
    let arr = [-2, -3, 4, -1, -2, 1, 5, -3];
    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}