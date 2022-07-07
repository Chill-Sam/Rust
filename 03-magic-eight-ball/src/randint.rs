use rand::{thread_rng, Rng};

// Random int
pub fn randint(a: u8, b: u8) -> u8
{
    let mut rng = thread_rng();

    // Random int between 2 args INCLUSIVE
    let randomint: u8 = rng.gen_range(a..=b);
    
    // return random int
    return randomint;
}