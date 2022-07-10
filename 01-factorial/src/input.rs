use std::io;

pub fn intinput() -> i128
{
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Could not read from stdin.");
    let num: i128 = input.trim().parse().expect("Not a valid number.");
    return num;
}