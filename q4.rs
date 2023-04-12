fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let limit = (n as f64).sqrt() as u64 + 1;
    for i in 2..limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let a = 19;
    println!("{:?}",is_prime(a));
}