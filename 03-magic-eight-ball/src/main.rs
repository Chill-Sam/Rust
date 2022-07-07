mod randint;
mod randans;
use std::io;

fn main() 
{
    // Get reduntant input
    println! ( "Enter your question: " );
    let mut reduntantinput = String::new(); io::stdin().read_line(&mut reduntantinput).expect("Couldn't read from stdin.");

    // Random number and then use that random number to refrence a answer.
    let randomint = randint::randint(1, 20);
    let randomans = randans::answer(randomint);

    // Print answer
    println! ( "{}", randomans );
}
