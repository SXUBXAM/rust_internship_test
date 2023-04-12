fn short(s: &str) -> Option<&str> {
    let words = s.split_whitespace();
    let mut shortest: Option<&str> = None;
    
    for word in words {
        if shortest.is_none() || word.len() < shortest.unwrap().len() {
            shortest = Some(word);
        }
    }
    
    shortest
}

fn main() {
    let s = "this is the best opportunity a";
    let short = short(s);
    println!("{:?}",short); 
}