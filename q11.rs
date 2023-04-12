fn merge(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;
    
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }
    result
}

fn main() {
    let arr1 = [1,3,5,7,9,11];
    let arr2 = [0,2,4,6,8,10];
    
    println!("{:?}",merge(arr1.to_vec(), arr2.to_vec()));
    
}