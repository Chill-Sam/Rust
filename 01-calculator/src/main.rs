mod input;
mod operations;

fn main() 
{
    println! ( "Pick first number: " ); let input1: f64 = input::numinput(); 
    println! ( "Pick second number: " ); let input2: f64 = input::numinput();
    println! ( "Pick a operation. (+, -, *, /)" ); let operation: u16 = input::operationinput();

    if operation == 1 {let answer: f64 = operations::add(input1, input2); println!( "{} + {} = {}", input1, input2, answer ); return;}
    if operation == 2 {let answer: f64 = operations::sub(input1, input2); println!( "{} - {} = {}", input1, input2, answer ); return;}
    if operation == 3 {let answer: f64 = operations::mul(input1, input2); println!( "{} * {} = {}", input1, input2, answer ); return;}
    if operation == 4 {let answer: f64 = operations::div(input1, input2); println!( "{} / {} = {}", input1, input2, answer ); return;}
    if operation == 404 {main();}
    else {println!( "Error, something went wrong." ); main()}
}