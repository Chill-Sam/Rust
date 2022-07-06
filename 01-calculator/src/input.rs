use std::io;


pub fn numinput() -> f64
{
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin.");
    let float: f64 = input.trim().parse().expect("Not a valid number.");
    return float;
}

pub fn operationinput() -> u16
{
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin.");
    let operation: char = input.trim().parse().expect("Not a valid char.");
    if operation == '+'{return 1;} if operation == '-'{return 2;} if operation == '*'{return 3;} if operation == '/'{return 4;}
    else {println! ( "Not a operation." ); return 404;}
}