use rand::{thread_rng, Rng};
use std::io;

fn main() 
{
    let mut win: bool = false;
    let random_num: i128 = randint(1, 100);
    let mut guess: i128;

    while win == false
    {
        println! ( "Guess a number between 1 and 100." );
        guess = guess_number();
        if guess < 1 {println! ("That is outside of the range.");}
        if guess > 100 {println! ("That is outside of the range.");}
        if guess == random_num {win = true;}
        if guess > random_num {println! ("Less!");}
        if guess < random_num {println! ("More!");}
    }
    
    println! ( "You win!!! The number was {random_num}." );
}

fn guess_number() -> i128
{
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    
    let num: i128 = input
        .trim()
        .parse()
        .expect("Not a valid number (128 bit integer)");
    
    return num;
}

// Random int
fn randint(a: i128, b: i128) -> i128
{
    let mut rng = thread_rng();

    // Random int between 2 args INCLUSIVE
    let randomint: i128 = rng.gen_range(a..=b);
    
    // return random int
    return randomint;
}