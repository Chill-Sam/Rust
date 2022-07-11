// Takes an input and reverses it recursivly.

use std::io;

fn main() 
{
    print!("{esc}c", esc = 27 as char);
    println! ( "Please enter a string." );
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    
    let string_vector: Vec<char> = input.trim().chars().collect();
    print!("{esc}c", esc = 27 as char);
    println! ( "Original: {}\nReverse: {}", string_vector.iter().collect::<String>(), reverse(string_vector).iter().collect::<String>() )
}

fn reverse(x: Vec<char>) -> Vec<char>
{
    let mut reverse_x: Vec<char> = Vec::new();
    let i: usize = 0;

    for i in 0..x.len()
    {
        reverse_x.push(x[x.len() - 1 - i]);
    }
    reverse_x
}
