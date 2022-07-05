pub fn run() {
    // Prints the line "Hello world!"
    println!("Hello world!");

    // Prints with place holders
    println!("Number: {}, Letter: {}", 1, "a");

    // Positional arguments
    println!("Number: {0}, Letter: {1}, Number again: {0}", 1, "a");

    // Named arguments
    println!("{name} likes to {activity}.", name = "John", activity = "play");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Basic math
    println!("10 + 10 = {}", 10 + 10);

}