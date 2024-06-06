fn find_median(arr: &[i32]) -> f64 {
    let n = arr.len();
    let mut arr = arr.to_vec();
    arr.sort();
    if n % 2 == 0 {
        let mid1 = arr[n / 2 - 1] as f64;
        let mid2 = arr[n / 2] as f64;
        (mid1 + mid2) / 2.0
    } else {
        arr[n / 2] as f64
    }
}

fn main() {
    let arr = [15, 21, 57, 27, 69];
    let median = find_median(&arr);
    println!("The median of the array is: {}", median);
}