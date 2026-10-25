use std::io::Write;

fn main() {
   let header = r"Commissioner\Ministry\Zone";
   
   let names = vec![
      "Aigbogun Alamba Dauda", 
      "Murtala Afeez Bendu",
      "Okorocha Calistus Ogbona",
      "Adewale Jimoh Akanbi",
      "Osazuwa Faith Etieye"
   ];
   let ministries = vec![
      "Internal Affairs",
      "Justice",
      "Defense",
      "Power & Steel", 
      "Petroleum",
   ];
   let zones = vec![
      "South West", 
      "North East", 
      "South South", 
      "South West", 
      "South East",
   ];


   
   println!("{}", header);
   println!();
   for i in 0..names.len() {
   println!("{:?},{:?},{:?}", names[i], ministries[i], zones[i]);
   }

   let mut file = std::fs::File::create("efcc_merge.txt ").expect("create failed");
   file.write_all(header.as_bytes()).expect("create failed");

   println!("\nData written successfully to efcc_merge.txt");

}
