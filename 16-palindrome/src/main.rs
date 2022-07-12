use std::io;

fn main() 
{
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let string_vector: Vec<char> = input.trim().chars().collect();

    let vector_alphanumeric = copy_alphanumeric(&string_vector);

    if vector_alphanumeric.iter().collect::<String>().to_lowercase() == reverse(&vector_alphanumeric).iter().collect::<String>().to_lowercase()
    {
        println!("{} is a palindrome!", string_vector.iter().collect::<String>());
    }
    else
    {
        println!("{} is NOT a palindrome!", string_vector.iter().collect::<String>());
    }
}
 
fn reverse(x: &Vec<char>) -> Vec<char>
{
    let mut reverse_x: Vec<char> = Vec::new();

    for i in 0..x.len()
    {
        reverse_x.push(x[x.len() - 1 - i]);
    }
    reverse_x
}

fn copy_alphanumeric(x: &Vec<char>) -> Vec<char>
{
    let mut copy_x: Vec<char> = Vec::new();

    for i in 0..x.len()
    {
        if x[i].is_ascii_alphanumeric()
        {
            copy_x.push(x[i])
        }
    }
    copy_x
}