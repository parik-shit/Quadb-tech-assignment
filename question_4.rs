fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let number = 17;
    if is_prime(number) {
        println!("{} is prime.", number);
    } else {
        println!("{} is not prime.", number);
    }
}
