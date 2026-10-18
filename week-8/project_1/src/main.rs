use std::io;

fn main() {
    // public servant categories vector
    let categories = vec!["Lawyer", "Teacher", "Academic", "Office Admin"];

    let mut role = String::new();
    println!("Choose a public servant category (0:Lawyer, 1:Teacher, 2:Academic, 3:Office Administration)");
    io::stdin().read_line(&mut role).expect("Failed to read input");
    let role_index: usize = role.trim().parse().expect("Invalid input, please enter a number");

    let mut years = String::new();
    println!("Enter your years of experience:");
    io::stdin().read_line(&mut years).expect("Failed to read input");
    let years: u32 = years.trim().parse().expect("Invalid input, please enter a number");

    let chosen_role = categories[role_index];

    // Simple evaluation logic based on the table
    let level = match chosen_role {
        "Lawyer" => {
            if years <= 2 {"APS 1-2"}
            else if years <= 5 {"APS 3-5"}
            else if years <=8 {"APS 5-8"}
            else {"EL1/EL2"}
        }
        _ => "General Public Service Level"
    };
    println!("\nCategories: {}", chosen_role);
    println!("Years of experience: {}", years);
    println!("Assigned Public Service Level: {}\n", level);



}
