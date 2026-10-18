fn main() {
    //candidates vector
    let candidates = vec![("Elsie", 10), ("Chinyere", 6), ("Anu", 8), ("Edwina", 13), ("Emmanuel", 5)];


   let mut top_candidate = candidates[0];

   for candidate in &candidates {
      if candidate.1 > top_candidate.1 {
          top_candidate = *candidate; 
      }
   }

   println!("EY Nigeria Developer Recruitment");
   println!("The developer with the  highest programming experience is {} with {} years of experience", top_candidate.0, top_candidate.1);


}
