use std::io;

fn choice_input(prompt: &str) -> f32 {
    println!("{}",prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("This is not a valid string");
    let user_input:f32 = input.trim().parse().expect("This is not an integer");
    user_input
}

fn trapezium_area(height:f32, base1:f32, base2:f32) -> f32 {
    (height/2.0)*(base1 + base2)
}

fn rhombus_area(d1:f32, d2:f32) -> f32 {
    0.5*d1*d2
}

fn parallelogram_area(base:f32, alt:f32) -> f32 {
    base*alt
}

fn cube_area(length:f32) -> f32 {
    6.0*(length.powf(2.0))
}

fn cylinder_volume(radius:f32, height:f32) -> f32 {
    3.142*(radius.powf(2.0))*height
}

fn main() {
    println!("\n============Geometry Formula Calculator=============");
    println!("\nHello, user, kindly choose your desired calculation:");
    println!("\n1. Area of a trapezium");
    println!("2. Area of a rhombus");
    println!("3. Area of a parallelogram");
    println!("4. Area of a cube");
    println!("5. Volume of a cylinder");

    println!("\nYour choice:");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("This is not a valid string");
    let choice:u8 = choice.trim().parse().expect("This is not a valid number");

    match choice {
       1 => {
        let height = choice_input("\nEnter the height of the trapezium:");
        let base1 = choice_input("\nEnter the top side of the trapezium:");
        let base2 = choice_input("\nEnter the bottom side of the trapezium:");
        let area = trapezium_area(height, base1, base2);
        println!("\nThe area of the trapezium is: {:.2}",area);
       }
       2 => {
        let d1 = choice_input("\nEnter the first diagonal of the rhombus:");
        let d2 = choice_input("\nEnter the second diagonal of the rhombus:");
        let area = rhombus_area(d1, d2);
        println!("\nThe area of the rhombus is: {:.2}",area);
       }
       3 => {
        let base = choice_input("\nEnter the base of the parallelogram:");
        let alt = choice_input("\nEnter the altitude of the parallelogram:");
        let area = parallelogram_area(base, alt);
        println!("\nThe area of the parallelogram is: {:.2}",area);
       }
       4 => {
        let length = choice_input("\nEnter the length of the cube:");
        let area = cube_area(length);
        println!("\nThe area of the cube is: {:.2}",area);
       }
       5 => {
        let radius = choice_input("\nEnter the radius of the cylinder:");
        let height = choice_input("\nEnter the height of the cylinder:");
        let volume = cylinder_volume(radius, height);
        println!("\nThe volume of the cylinder is: {:.2}",volume);
       }
       _ => {
        println!("\nInvalid input! Please enter a number between 1 and 5.");
       }
    }
}
