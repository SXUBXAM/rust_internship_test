fn smallest(n: &[i32], k: usize) -> i32 {
    let mut n = n.to_vec();
    n.sort();
    n[k-1]
}

fn main() {
    let a = [1,5,6,8,3,7];
    println!("{:?}",smallest(&a, 2));
}