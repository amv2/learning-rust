use std::io;
use std::f64::consts::PI;
use std::f64::consts::SQRT_2;

fn main() {

    loop {
        // greet the user
        println!("Calculate the area!");

        // get the user input for the shape
        println!("What shape?");
        println!("1: Circle");
        println!("2: Triangle");
        println!("3: Rectangle");
        println!("4: Pentagon");

        let mut choice = String::new();

        io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You chose: {choice}");

        // get the user input for length of one side (radius if circle)
        println!("What is the length of one side? (len of radius if circle)");

        let mut side_length = String::new();

        io::stdin()
                .read_line(&mut side_length)
                .expect("Failed to read line");

        let side_length: f64 = match side_length.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Side is : {side_length} units");

        // calculate the area
        let mut area: f64 = 0.0;

        if choice == 1 {
            area = PI * side_length * side_length;
        } else if choice == 2 {
            area = ((3.0_f64).sqrt() / 4.0) * side_length * side_length; 
        } else if choice == 3 {
            area = side_length * side_length;
        } else if choice == 4 {
            area = 0.25 * (5.0 * (5.0 + 2.0 * (5.0_f64).sqrt())).sqrt() * side_length * side_length;
        } else {
            println!("Invalid option!");
        }

        // print the area
        println!("Area is : {area} units squared");
    }
}
