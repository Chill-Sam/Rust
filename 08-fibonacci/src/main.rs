use std::io;

fn main() 
{
    println!("Please enter a number: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let num: u32 = input
        .trim()
        .parse()
        .expect("Not a valid number.");
    
        let mut prev_num = 1;
        let mut cur_num = 1;
        let mut next_num;
        println! ("Fibonacci sequence until term {num}.");
        println! ("{prev_num}");
        println! ("{cur_num}");
        
        let mut n: u32 = 3;

        while n <= num
        {
            n = n + 1;
            next_num = prev_num + cur_num;
            prev_num = cur_num;
            cur_num = next_num;
            println! ("{}", next_num);
        } 
    } 
