pub fn run() 
{
    let name = "Jonathan";
    let mut age = 13;

    println! ( "My name is {} and I am {}.", name, age );

    age = 14;
    
    println! ( "Next year I will be {} years old.", age );

    // Define constant
    const ID: i32 = 001;

    println! ("ID: {}", ID);

    // Assign multiple vars
    let ( my_name, my_age ) = ("Jonathan", 13);
    println! ( "My name is {} and I am {}.", my_name, my_age );

}