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
    print!("{esc}c", esc = 27 as char);
    loop 
    {
        // Display options
        println! ( "Choose an option. (X to exit):" );
        println! ( "(1) - SEK to euros" );
        println! ( "(2) - Euros to SEK" );
        
        // Get input
        let mut choice_input: String = String::new();
        io::stdin()
            .read_line(&mut choice_input)
            .expect("Failed to read input.");

        // Variables for choice
        let type_of_currency: &str;
        let symbol_of_currency: String;
        let converted_symbol: String;

        match choice_input.as_str()
        {
            "X\n" => 
            {
                print!("{esc}c", esc = 27 as char);
                return;
            }
            _ => ()
        }

        // parse to u8 with match statement
        match choice_input.trim().parse::<u8>() {
            Ok(choice) => {
                // Match statement to check choice  
                match choice {
                    1 => {
                        print!("{esc}c", esc = 27 as char);
                        println!( "Enter amount in SEK" );
                        type_of_currency = "SEK";
                        symbol_of_currency = String::from("kr");
                        converted_symbol = String::from("€");
                        choice
                    }
                    2 => {
                        print!("{esc}c", esc = 27 as char);
                        println!( "Enter amount in Euro" );
                        type_of_currency = "EUR";
                        symbol_of_currency = String::from("€");
                        converted_symbol = String::from("kr");
                        choice
                    }
                    
                    _ => {println!( "Input is not valid." ); print!("{esc}c", esc = 27 as char); continue}, // Invalid input
                }
            }
            Err(_) => {println!( "Input is not valid." ); print!("{esc}c", esc = 27 as char); continue}, // Invalid input
        };

        // Get input for amount
        let mut amount_input: String = String::new();
        io::stdin()
            .read_line(&mut amount_input)
            .expect("Failed to read input.");

        match amount_input.trim().parse::<f64>()
        {
            Ok(amount) => 
            {
                match amount 
                {
                    x if x > 0.0 =>
                    {
                        match type_of_currency 
                        {
                            "SEK" => 
                            {
                                print!("{esc}c", esc = 27 as char);
                                println! ("{} {} is {}{}\n", amount, symbol_of_currency, converted_symbol, convert(amount, &type_of_currency)); 
                            }
                            "EUR" => 
                            {
                                print!("{esc}c", esc = 27 as char);
                                println! ("{}{} is {} {}\n", symbol_of_currency, amount, convert(amount, &type_of_currency), converted_symbol); 
                            }
                            _ => panic! ( "A fatal error has occured." ),
                        }
                    }
                    _ => {println!( "Input is not valid." ); print!("{esc}c", esc = 27 as char); continue},
                } 
            }
            Err(_) => {println!( "Failed to parse to f64." ); print!("{esc}c", esc = 27 as char); continue},
        };
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