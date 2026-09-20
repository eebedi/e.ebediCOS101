// Rust program to output name and age

use std::io;

fn main() {
	println!("Is the employee experienced? (enter 'yes' or 'no'):");
	let mut exp_input = String::new();
	io::stdin().read_line(&mut exp_input).expect("Failed to read input");
	let is_experienced = exp_input.trim().to_lowercase() == "yes";

	println!("Enter the employee's age:");
	let mut age_input = String::new();
	io::stdin().read_line(&mut age_input).expect("Failed to read input")
	let age: u32 = age_input.trim().parse().expect("Failed to input");

	//determine incentive based on criteria
	let incentive = if is_experienced {
		if age >= 40 {
			1_560_000
		} else if age >= 30 && age <= 39 {
			1_480_000
		} else if age < 28 {
			1_300_000
		} else {
			//default rate for experienced employess aged 28 or 29
			1_300_000
		}
	} else {
		100_000
	};
	println!("The annual incentive is: N{}", incentive);
}