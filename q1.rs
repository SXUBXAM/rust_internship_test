fn is_pal(s: &str) -> bool {
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}

fn main(){
    let s = "mississim";

    let is_p = is_pal(s);
    println!("{:?}", is_p)
}