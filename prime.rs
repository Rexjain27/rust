fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    let num = 27;
    if is_prime(num) {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);
    }
}