// Calculate discriminant d = b^2 - 4 * a * c
use std::io;
fn main() {
  println!("Enter value for a:");
   let mut input1 = String::new();
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let a: f64 = input1.trim().parse().expect("Failed to input");

   println!("Enter value for b:");
   let mut input2 = String::new();
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let b: f64 = input2.trim().parse().expect("Failed to input");

   println!("Enter value for c:");
   let mut input3 = String::new();
   io::stdin().read_line(&mut input3).expect("Failed to read input");
   let c: f64 = input3.trim().parse().expect("Failed to input");

  // Calculate discriminant d = b^2 - 4ac
    let d = b * b - 4.0 * a * c;

  // Determine roots based on the value of the discriminant if d > 0.0 {
    let root1 = (-b + d.sqrt()) / (2.0 * a);
    let root2 = (-b - d.sqrt()) / (2.0 * a);
    println!("There are two distinct roots: {} and {}", root1, root2);
    if d == 0.0 {
      let _roots = -b / (2.0 * a);}
      println!("There is exactly one real root",);
    {
      println!("There are no real roots");
  }
}

    
    
    
    