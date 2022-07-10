// Function to give a answer depending on input.
pub fn answer(a: u8) -> String
{
    // Answer string
    let mut ans = String::new();
    
    // Positive answers
    if a == 1 {ans = "It is certain.".to_string();} if a == 2 {ans = "It is decidedly so.".to_string();} if a == 3 {ans = "Without a doubt.".to_string();} if a == 4 {ans = "Yes definitely.".to_string();} if a == 5 {ans = "You may rely on it.".to_string();}
    if a == 6 {ans = "As I see it, yes.".to_string();} if a == 7 {ans = "Most likely.".to_string();} if a == 8 {ans = "Outlook good.".to_string();} if a == 9 {ans = "Yes.".to_string();} if a == 10 {ans = "Signs point to yes.".to_string();}

    // Non committal answers
    if a == 11 {ans = "Reply hazy, try again.".to_string();} if a == 12 {ans = "Ask again later.".to_string();} if a == 13 {ans = "Better not tell you now.".to_string();} if a == 14 {ans = "Cannot predict now.".to_string();} if a == 15 {ans = "Concentrate and ask again.".to_string();}

    // Negative answers
    if a == 16 {ans = "Don't count on it.".to_string();} if a == 17 {ans = "My reply is no.".to_string();} if a == 18 {ans = "My sources say no.".to_string();} if a == 19 {ans = "Outlook not so good.".to_string();} if a == 20 {ans = "Very doubtful.".to_string();}

    // Return answer
    return ans;
}