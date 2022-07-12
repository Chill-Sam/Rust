use std::io;

fn main() 
{
    // unit_test();

    println!("How many £2 coins: ");
    let two_pound: u64 = input();

    println!("How many £1 coins: ");
    let one_pound: u64 = input();

    println!("How many 50p coins: ");
    let fifty_p: u64 = input();

    println!("How many 20p coins: ");
    let twenty_p: u64 = input();

    println!("How many 10p coins: ");
    let ten_p: u64 = input();
    
    println!("How many 5p coins: ");
    let five_p: u64 = input();

    let total: f64 = answer(two_pound as f64, one_pound as f64, fifty_p as f64, twenty_p as f64, ten_p as f64, five_p as f64);
    
    println! ( "Total: {}", total );
}

fn answer(two_pound: f64, one_pound: f64, fifty_p: f64, twenty_p: f64, ten_p: f64, five_p: f64) -> f64
{
    let total: f64 = two_pound * 2.0 + one_pound + fifty_p * 0.5 + twenty_p * 0.2 + ten_p * 0.1 + five_p * 0.05;

    total
}


fn input() -> u64
{
    let mut x: String = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read from input.");
    
    let format_x: u64 =
    match x.trim().parse::<u64>()
    {
        Ok(x) => x,
        Err(_) => panic! ( "Input is not valid." ), 
    };
    format_x
}

// TESTS

fn unit_test()
{
    let answer = answer(1 as f64, 3 as f64, 5 as f64, 2 as f64, 1 as f64, 15 as f64);

    if answer == 8.75 {
        println! ( "Test passed!" )
    }
    else {
        println! ( "Test failed! {}", answer )
    }
}