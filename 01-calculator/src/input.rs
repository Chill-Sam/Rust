use std::io;


pub fn run() -> f64
{
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Not a valid string.");
    let float: f64 = input.trim().parse().expect("Not a valid number");
    return float;
}