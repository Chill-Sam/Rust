fn main() 
{
    let mut sum: u128 = 0;
    
    let mut i = 1;
    while i < 100
    {
        println! ("{i}");
        sum += i;
        i += 2;
    }
    println! ("{sum}");
    println! ("{}", fifthroot(sum as f64));
}

fn fifthroot(num: f64) -> f64
{
    return num.powf(1.0/5.0);
}