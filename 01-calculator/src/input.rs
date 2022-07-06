use std::io;

// Returns f64 input
pub fn numinput() -> f64
{
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let float: f64 = input.trim().parse().expect("Not a valid float."); // attemps to parse to f64. if failure, it is not a float
    return float;
}

// Takes input for an operation and returns a u16
pub fn operationinput() -> u16
{
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let operation: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failute, it is not a char.
    if operation == '+'{return 1;} if operation == '-'{return 2;} if operation == '*'{return 3;} if operation == '/'{return 4;} // If statements to return u16 depending on operation chosen.
    else {println! ( "Not a operation." ); return 404;} // Returns 404 if input is not an operation
}