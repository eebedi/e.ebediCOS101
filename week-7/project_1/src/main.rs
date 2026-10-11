use std::io;

//Trapezium Area
fn calculate_trapezium() -> f64 {
    let mut height = String::new();
    let mut base1 = String::new();
    let mut base2 = String::new();

    println!("Enter height:");
    io::stdin().read_line(&mut height).expect("Failed to read line");
    let h: f64 = height.trim().parse().expect("Please enter a valid number");

    println!("Enter base1:");
    io::stdin().read_line(&mut base1).expect("Failed to read line");
    let b1: f64 = base1.trim().parse().expect("Please enter a valid number");

    println!("Enter base2:");
    io::stdin().read_line(&mut base2).expect("Failed to read line");
    let b2: f64 = base2.trim().parse().expect("Please enter a valid number");

    h / 2.0 * (b1 + b2)
}

// Rhombus Area
fn calculate_rhombus() -> f64 {
    let mut diag1 = String::new();
    let mut diag2 = String::new();

    println!("Enter diagonal 1:");
    io::stdin().read_line(&mut diag1).expect("Failed to read line");
    let d1: f64 = diag1.trim().parse().expect("Please enter a valid number");

    println!("Enter diagonal 2:");
    io::stdin().read_line(&mut diag2).expect("Failed to read line");
    let d2: f64 = diag2.trim().parse().expect("Please enter a valid number");

    0.5 * d1 * d2
}

// Parallelogram Area
fn calculate_parallelogram() -> f64 {
    let mut base = String::new();
    let mut altitude = String::new();

    println!("Enter base:");
    io::stdin().read_line(&mut base).expect("Failed to read line");
    let b: f64 = base.trim().parse().expect("Please enter a valid number");

    println!("Enter altitude:");
    io::stdin().read_line(&mut altitude).expect("Failed to read line");
    let a: f64 = altitude.trim().parse().expect("Please enter a valid number");

    b * a
}

// Cube Surface Area
fn calculate_cube() -> f64 {
    let mut side = String::new();

    println!("Enter side length:");
    io::stdin().read_line(&mut side).expect("Failed to read line");
    let s: f64 = side.trim().parse().expect("Please enter a valid number");

    6.0 * s * s
}

// Cylinder Volume
fn calculate_cylinder() -> f64 {
    let mut radius = String::new();
    let mut height = String::new();

    println!("Enter radius:");
    io::stdin().read_line(&mut radius).expect("Failed to read line");
    let r: f64 = radius.trim().parse().expect("Please enter a valid number");

    println!("Enter height:");
    io::stdin().read_line(&mut height).expect("Failed to read line");
    let h: f64 = height.trim().parse().expect("Please enter a valid number");

    let pi = 3.14159;
    pi * r * r * h
}

fn main() {
    println!("Select a shape to calculate:");
    println!("1. Trapezium Area");
    println!("2. Rhombus Area");
    println!("3. Parallelogram Area");
    println!("4. Cube Surface Area");
    println!("5. Cylinder Volume");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read choice");
    let option: i32 = choice.trim().parse().expect("Please enter a valid number");

    if option == 1 {
        let area = calculate_trapezium();
        println!("The area of the trapezium is: {}", area);
    } else if option == 2 {
        let area = calculate_rhombus();
        println!("The area of the rhombus is: {}", area);
    } else if option == 3 {
        let area = calculate_parallelogram();
        println!("The area of the parallelogram is: {}", area);
    } else if option == 4 {
        let area = calculate_cube();
        println!("The surface area of the cube is: {}", area);
    } else if option == 5 {
        let volume = calculate_cylinder();
        println!("The volume of the cylinder is: {}", volume);
    } else {
        println!("Invalid selection!");
    }
}