mod input;
mod operations;

// main calculator function
fn main() 
{
    println! ( "Pick first number: " ); let input1: f64 = input::numinput(); // First number input 
    println! ( "Pick second number: " ); let input2: f64 = input::numinput(); // Second number input
    println! ( "Pick a operation. (+, -, *, /)" ); let operation: u16 = input::operationinput(); // Operator input

    // if statements to find which operation and displays answer.
    if operation == 1 {let answer: f64 = operations::add(input1, input2); println!( "{} + {} = {}", input1, input2, answer ); return;} // Addition
    if operation == 2 {let answer: f64 = operations::sub(input1, input2); println!( "{} - {} = {}", input1, input2, answer ); return;} // Subtraction
    if operation == 3 {let answer: f64 = operations::mul(input1, input2); println!( "{} * {} = {}", input1, input2, answer ); return;} // Multiplication
    if operation == 4 {let answer: f64 = operations::div(input1, input2); println!( "{} / {} = {}", input1, input2, answer ); return;} // Division

    if operation == 404 {main();} // restarts program if operation returns 404. (If operation choice isnt a valid operator.)
    else {println!( "Error, something went wrong." ); main()} // If something goes very wrong with operation then restart program.
}