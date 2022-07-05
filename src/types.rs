pub fn run()
{
    // Add explicit variables
    let x: i64 = 129391023891238;

    // Find max size
    println! ( "Max i32: {}", std::i32::MAX );
    println! ( "Max i64: {}", std::i64::MAX );

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    // Char
    let a1 = 'a';
    let face = '\u{1F600}';

    println! ( "{:?}", (x, is_active, is_greater, a1, face) );
}