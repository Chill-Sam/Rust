use std::io;

fn main() 
{
    println! ( "Enter a number: " );

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    
    let max_num: u16 = input
        .trim()
        .parse()
        .expect("Not a valid number (positive 16 bit integer)");
    
    let mut num = 1;
    while num <= max_num
    {
        let print = "*".repeat(num.into());
        println! ("{}", print);
        num = num + 2;
    }
}
