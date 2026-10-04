use std::io;
fn main() {


    //display of the menu
    println!("WELCOME TO AMALA BASE");
    println!("What would you like to order?");
    println!("MENU");
    println!("P - Poundo Yam/Edinkaiko Soup #3200");
    println!("F - Fried Rice & Chicken      #3000");
    println!("A - Amala & Ewedu             #2500");
    println!("E - Eba & Egusi Soup          #2000");
    println!("W - White Rice & Stew         #2500");

    //input order
    println!("\n Enter order here (P, F, A, E, W):");
    let mut food_input = String::new(); 
    io::stdin().read_line(&mut food_input).unwrap(); 
    let food = food_input.trim(); 

    // 3. Decision on the letter using if / else if and || (Logical OR)
    let mut price = 0.0;
    if food == "P" || food == "p" {
        price = 3200.0;
    } else if food == "F" || food == "f" {
        price = 3000.0;
    } else if food == "A" || food == "a" {
        price = 2500.0;
    } else if food == "E" || food == "e" {
        price = 2000.0;
    } else if food == "W" || food == "w" {
        price = 2500.0;
    }

    // Read the quantity
    println!("Enter quantity:");
    let mut qty_input = String::new();
    io::stdin().read_line(&mut qty_input).unwrap();
    let qty: f32 = qty_input.trim().parse().unwrap();

    // Arithmetic operator (*) for the total
    let mut total = price * qty;

    // One last if statement for the discount
    if total > 10000.0 {
        total = total - (total * 0.05); // Deduct 5%
    }

    //Output total charge
    println!("Total Charge: N{}", total);
}

 