use std::io;

fn main() 
{
    println! ( "Enter a 4 letter word: " );

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input = input
        .trim()
        .to_string()
        .to_uppercase();
    
    if input.len() != 4 {panic! ("Input is not 4 characters")} 

    let all_perms = permutation(input);
    let mut unique_perms = all_perms.clone();

    unique_perms.sort();
    unique_perms.dedup();

    println!("List of permutations: {:?}", unique_perms);
    println!("Permutations: {}", unique_perms.len());

}

// from https://stackoverflow.com/questions/14008521/please-explain-this-algorithm-to-get-all-permutations-of-a-string
fn permutation(word: String) -> Vec<String>
{
    if word.len()<=1 
    {
        return vec![word];
    }

    let trimmed_word = word.chars().skip(1).collect();
    let perms = permutation(trimmed_word);
    let character = word.chars().nth(0).unwrap();
    let mut result_vec = Vec::new(); 
    
    for perm in &perms
    {
        for i in 0..&perms.len() + 1
        {
            let upper: String = perm.chars().take(i).collect();
            let lower: String = perm.chars().skip(i).collect();
            result_vec.push(format!("{}{}{}", upper, character, lower ));
        }
    }
    return result_vec;
    
}