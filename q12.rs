fn subarrsum(arr: &[i32]) -> i32 {
    let mut maxs = arr[0];
    let mut mh = arr[0];
    for &num in arr.iter().skip(1) {
        mh = std::cmp::max(num, mh + num);
        maxs = std::cmp::max(maxs, mh);
    }
    maxs
}

fn main() {
    let a = [1,3,5,-5,4,-8,2,10];
    println!("{:?}",subarrsum(&a));
}