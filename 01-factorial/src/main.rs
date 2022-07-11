use std::io;

fn main() 
{
    print!("{esc}c", esc = 27 as char);
    loop
    {
        // Gets input
        println!("Pick a number or leave (X to leave): ");   
        let mut num_input: String = String::new();
        io::stdin()
            .read_line(&mut num_input)
            .expect ( "Failed to read from stdin!" );

        // Leaves
        match num_input.as_str()
        {
            "X\n" => {
                print!("{esc}c", esc = 27 as char);
                return;
            }
            _ => ()

        }

        // Max 34 else overflow
        let num: u128 = 
        match num_input.trim().parse()
        {
            Ok(num) => {
                match num {
                    // Normal Cases
                    1..=34 => num,

                    // 0 Case
                    0 => {
                        print!("{esc}c", esc = 27 as char);
                        println! ( "{}! = 1", num ); 
                        continue;
                    }

                    // Too large
                    35.. => {
                        print!("{esc}c", esc = 27 as char);
                        println! ( "Number too large!" );
                        continue;
                    }
                }
            }
            // Not a number
            Err(_) => {
                print!("{esc}c", esc = 27 as char);
                println! ( "Not a number!" );
                continue;
            }
        };

        // calculates answer
        let mut answer: u128 = num;
        let mut i: u128 = num - 1;
        
        while i > 0
        {
            answer *= i;
            i -= 1;
        }

        print!("{esc}c", esc = 27 as char);
        println! ( "{}! = {}", num, answer );
    }
}   
