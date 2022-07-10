mod input;

fn main() 
{
    println! ( "Pick a number: " );
    let num = input::intinput();

    if num > 33 {println! ( "Number too large. (MAX = 33)" ); return; }
    if num == 0 {println! ( "0! = 1" ); return;}
    if num < 0 {println! ( "Cannot take factorial of negative number" ); return;}

    let mut factorial: i128 = num.into();
    let mut numfactorial: i128 = (num - 1).into();
    
    while numfactorial > 0
    {
        factorial = factorial * numfactorial;
        numfactorial = numfactorial - 1;
    }
    println! ( "{}! = {}", num, factorial );
    return;
}
