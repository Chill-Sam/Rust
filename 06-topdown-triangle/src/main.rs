use std::io;

fn main() 
{
    println! ( "Enter a number: " );

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    
    let mut max_num: u16 = input
        .trim()
        .parse()
        .expect("Not a valid number (positive 16 bit integer)");
    
    while max_num > 0
    {
        let print = "*".repeat(max_num.into());
        println! ("{}", print);
        max_num = max_num - 1;
    }
}
