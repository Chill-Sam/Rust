// Develop a programme to convert currency X to currency Y and vice versa
// 1 SEK = 0.0935 Euros
// 1 Euro = 10.6856 SEK
// As of Jul 10, 2022, 13:14 UTC

use std::io;

// Constants for conversion
const SEK_TO_EUR: f64 = 0.0935;
const EUR_TO_SEK: f64 = 10.6882;

// Main function
fn main() 
{
    // Display options
    println! ( "Choose an option. (type number to begin):" );
    println! ( "(1) - SEK to euros" );
    println! ( "(2) - Euros to SEK" );

    // Get input
    let mut choice_input: String = String::new();
    io::stdin()
        .read_line(&mut choice_input)
        .expect("Could not read from stdin");

    // parse to u8
    let choice: u8 = choice_input
        .trim()
        .parse()
        .expect("Input number too large (non-negative 8-bit integer)");

    // Variables for choice
    let type_of_currency: &str;
    let symbol_of_currency: String;
    let converted_symbol: String;
    
    // Match statement to check choice  
    match choice
    {
        1 => { println! ( "Enter amount in SEK" ); type_of_currency = "SEK"; symbol_of_currency = String::from("kr"); converted_symbol = String::from("€"); }
        2 => { println! ( "Enter amount in Euro" ); type_of_currency = "EUR"; symbol_of_currency = String::from("€"); converted_symbol = String::from("kr"); }
        _ => panic! ( "Not a valid input. (1, 2)" ),
    }

    // Get input for amount
    let mut amount_input: String = String::new();
    io::stdin()
        .read_line(&mut amount_input)
        .expect("Could not read from stdin");
    
    // Parse to f64 
    let amount: f64 = amount_input
        .trim()
        .parse()
        .expect("Not a valid amount (64-bit float)");

    if amount.is_sign_negative() {panic! ( "Not a valid amount (non-negative number)" ); } // Gaurd clause for negative numbers 

    // Prints result depending on type_of_currency
    match type_of_currency
    {
        "SEK" => {println! ("{} {} is {}{}", amount, symbol_of_currency, converted_symbol, convert(amount, &type_of_currency)); }
        "EUR" => {println! ("{}{} is {} {}", symbol_of_currency, amount, convert(amount, &type_of_currency), converted_symbol); }
        _ => panic! ( "A fatal error has occured." ),
    }
}

// Conversion function
fn convert(amount: f64, type_of_currency: &str) -> f64
{
    // Returns answer truncated to 2 decimals depending on type_of_currency
    match type_of_currency
    {
        "SEK" => return f64::trunc(amount * SEK_TO_EUR * 100.0) / 100.0,
        "EUR" => return f64::trunc(amount * EUR_TO_SEK * 100.0) / 100.0,
        _ => panic! ( "A fatal error has occured." ), 
    }
}