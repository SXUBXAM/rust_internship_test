fn index(arr: &[i32], x: i32) -> Option<usize> {
    let mut l = 0;
    let mut r = arr.len() - 1;
    
    while l <= r {
        let m = l + (r- l) / 2;
        
        if arr[m] == x {
            if m == 0 || arr[m - 1] < x {
                return Some(m);
            } else {
                r = m - 1;
            }
        } else if arr[m] < x {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    
    None
}

fn main() {
    let a:[i32; 7] = [1,3,5,5,2,32,5];
    let first_occ = index(&a,32);
    println!("{:?}",first_occ);
}