use std::io;
use std::str;
fn main() {
    let input: &str = "adbcdaDd";
    let mut _a = String::new();
    println!("Nhap 1 ki tu: ");
    io::stdin().read_line(&mut _a).expect("Failed to read line");
    let _a: char = _a.trim().parse().unwrap();
    let _lower = _a.to_lowercase().to_string();
    let _upper = _a.to_uppercase().to_string();
    let _result1 = str::replace(&input, &_upper, "");
    let _result2 = str::replace(&_result1, &_lower, "");
    println!("{}", _result2);
    println!("{}", input.len() - _result2.len());
}
