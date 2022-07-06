use std::io;

pub fn run() -> f64
{
    let mut meritvector = vec![swegrade(), enggrade(), mathgrade(), relgrade(), civgrade(), hisgrade(), geograde(), chemgrade(), phygrade(), biograde(), techgrade(), artgrade(), hkkgrade(), musgrade(), gymgrade(), craftgrade()];
    let mut modg = vec![modgrade()]; let othg = othgrade(); let mut othgv = vec![othg]; let minmerit = meritvector.iter().fold(f64::INFINITY, |a, &b| a.min(b));

    if minmerit < othg
    {
        let index = meritvector.iter().position(|x| *x == minmerit).unwrap(); 
        meritvector.remove(index); meritvector.append(&mut othgv);
    }
    meritvector.append(&mut modg);

    let mut merit: f64 = 0.0;  
    for grade in meritvector 
    {
        merit = merit + grade;
    }
    return merit; 
}

fn swegrade() -> f64
{
    println! ( "Swedish grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn enggrade() -> f64
{
    println! ( "English grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn mathgrade() -> f64
{
    println! ( "Math grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn relgrade() -> f64
{
    println! ( "Religion grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn civgrade() -> f64
{
    println! ( "Civics grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn hisgrade() -> f64
{
    println! ( "History grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn geograde() -> f64
{
    println! ( "Geography grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn chemgrade() -> f64
{
    println! ( "Chemistry grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn phygrade() -> f64
{
    println! ( "Physics grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn biograde() -> f64
{
    println! ( "Biology grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn techgrade() -> f64
{
    println! ( "Tech grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn artgrade() -> f64
{
    println! ( "Art grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn hkkgrade() -> f64
{
    println! ( "Home economics grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn musgrade() -> f64
{
    println! ( "Music grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not 'A' valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn gymgrade() -> f64
{
    println! ( "Gym grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn craftgrade() -> f64
{
    println! ( "Crafts grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn modgrade() -> f64
{
    println! ( "Modern language grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}

fn othgrade() -> f64
{
    println! ( "Other language grade: (A, B, C, D, E, F, X (Dont have))" );
    let mut input = String::new(); io::stdin().read_line(&mut input).expect("Error, could not read from stdin."); // takes string input from stdin
    let grade: char = input.trim().parse().expect("Not a valid char."); // attemps to parse to char. if failure, it is not a char.
    if grade == 'A' {return 20.0;} if grade == 'B' {return 17.5;} if grade == 'C' {return  15.0;} if grade == 'D' {return 12.5;} if grade == 'E' {return  10.0;} if grade == 'F' {return  0.0;} if grade == 'X' {return  0.0;}
    else {return 0.0;}
}