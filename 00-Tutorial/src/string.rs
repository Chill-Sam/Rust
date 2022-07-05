pub fn run()
{
    let mut hello = String::from ( "Hello " );

    // Get lenght
    println! ( "Length: {}", hello.len() );

    // Push char
    hello.push( 'w' );

    // Push string
    hello.push_str( "orld!" );

    // Capacity in bytes
    println! ( "Capacity: {}", hello.capacity() );

    // If empty
    println! ( "Is Empty: {}", hello.is_empty() );

    // Contains
    println! ( "Contains: {}", hello.contains( "world!") );

    // Replace
    println! ( "Replace: {}", hello.replace( "world!", "there!" ) );

    // Loop through whitespaces
    for word in hello.split_whitespace() 
    {
        println! ( "{}", word );
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push( 'a' );  
    s.push( 'b' );

    // Assertion testing
    assert_eq! ( 2, s.len() );
    assert_eq! ( 10, s.capacity() );

    println! ( "{}", hello );
}