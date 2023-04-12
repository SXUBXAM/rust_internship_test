fn main() {
    let s = String::from("i accepted my fate");
    let reverse = s.chars().rev().collect::<String>();
    println!("{}", reverse);
}

