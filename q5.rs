fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        
        (arr[len/2 - 1] + arr[len/2]) as f64 / 2.0
    } else {
        
        arr[len/2] as f64
    }
}

fn main() {
    let a = [1,2,3,5,6];
    println!("{:?}", median(&a));
}