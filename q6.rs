fn lcp(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    
    let min_len = strs.iter().map(|s| s.len()).min().unwrap();
    
    for i in 0..min_len {
        let c = strs[0].chars().nth(i).unwrap();

        if !strs.iter().all(|s| s.chars().nth(i).unwrap() == c) {
            return String::from(&strs[0][..i]);
        }
    }
    
    String::from(&strs[0][..min_len])
}

fn main() {

}