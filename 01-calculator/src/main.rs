mod input;
mod add;

fn main() 
{
    println! ( "Pick first number: " ); let input1: f64 = input::run(); 

    println! ( "Pick second number: " ); let input2: f64 = input::run();

    let answer: f64 = add::add(input1, input2); println!( "{} + {} = {}", input1, input2, answer );
}