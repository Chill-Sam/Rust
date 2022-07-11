use std::io;

fn main() 
{
    println! ( "Enter a word: " );
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");
    
    let mut vowels: u8 = 0;
    let mut consonants: u8 = 0;
    let vowel_list = ['A', 'E', 'I', 'O', 'U', 'Y'];
    for char in input.to_uppercase().chars()
    { 
        if char == ' ' {break;}
        if char.is_numeric() {break;}
        if vowel_list.contains(&char) {vowels = vowels + 1;}
        else {consonants = consonants + 1;}
    }
    println!("{vowels} vowels, {consonants} consonants.");
}
