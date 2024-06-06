fn kth_smallest(arr: &[i32], k: usize) -> i32 {
    let mut arr = arr.to_vec();
    arr.sort();
    arr[k - 1]
}

fn main() {
    let arr = [7, 10, 4, 3, 20, 15];
    let k = 5;
    let kth_smallest = kth_smallest(&arr, k);
    println!("The {}th smallest element is: {}", k, kth_smallest);
}