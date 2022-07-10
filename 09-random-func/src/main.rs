use rand::{thread_rng, Rng};

fn main() 
{
    let random: u8 = randint(1, 3);
    match random
    {
        1 => func1(),
        2 => func2(),
        3 => func3(),
        _ => panic! ("Something went wrong"),
    }
    return;
}

// Random int
fn randint(a: u8, b: u8) -> u8
{
    let mut rng = thread_rng();

    // Random int between 2 args INCLUSIVE
    let randomint: u8 = rng.gen_range(a..=b);
    
    // return random int
    return randomint;
}

fn func1()
{
    println!("Hello from function 1!");
}

fn func2()
{
    println!("Sup from function 2");
}

fn func3()
{
    println!("Hello world... was that correct?");
}