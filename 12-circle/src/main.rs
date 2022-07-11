use std::io;

#[derive(Debug)]
pub struct Circle
{
    radius: f64,
    area: f64,
    diameter: f64,
}
impl Circle
{
    fn radius_input(radius: f64) -> Circle {
        Circle {
            radius: radius, // Radius = radius
            area: std::f64::consts::PI * radius.powi(2), // Area = PI * radius squared
            diameter: 2_f64 * radius, // Diameter = 2 * radius
        }
    }

    fn area_input(area: f64) -> Circle {
        let radius: f64 = (area / std::f64::consts::PI).sqrt(); // Radius = sqrt(area / PI) 
        Circle {
            radius: radius, // radius = radius
            area: area, // area = area
            diameter: 2_f64 * radius, // diameter = 2 * radius
        }
    }

    fn diameter_input(diameter: f64) -> Circle {
        let radius: f64 = diameter / 2_f64; // radius = diameter / 2
        Circle {
            radius: radius, // radius = radius
            area: std::f64::consts::PI * radius.powi(2), // Area = PI * radius squared
            diameter: diameter, // diameter = diameter
        }
    }
}

fn main() 
{
    println!("Pick an option:");
    println!("1: Radius");
    println!("2: Area");
    println!("3: Diameter");

    let mut choice_input: String = String::new();
    io::stdin()
        .read_line(&mut choice_input)
        .expect("Could not read from stdin");

    let choice: u8 = match choice_input.trim().parse() {
        Ok(choice) => choice,
        Err(_) => panic! ( "That is not a number!" )
    };

    match choice {
        1 => println! ( "Enter radius: " ),
        2 => println! ( "Enter area: " ),
        3 => println! ( "Enter diameter: " ),
        _ => panic! ( "Pick a valid option!" )
    };

    let mut size_input: String = String::new();
    io::stdin()
        .read_line(&mut size_input)
        .expect("Failed to read input");
    
    match size_input.trim().parse() {
        Ok(size) => {
            let circle = match choice {
                1 => Circle::radius_input(size),
                2 => Circle::area_input(size),
                3 => Circle::diameter_input(size),
                _ => panic! ( "Something went wrong!" )
            };
            println!("Radius: {:?}", circle.radius);
            println!("Area: {:?}", circle.area);
            println!("Diameter: {:?}", circle.diameter);
        }
        Err(_) => panic! ( "That is not a number!" ),
    };
    
}
