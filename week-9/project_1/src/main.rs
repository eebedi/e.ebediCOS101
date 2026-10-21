use std::io::Write;

fn main() {
    
    let header = "Lager, Stout, Non-Alcoholic";
    let row1 = "33 Export, Legend, Maltina";
    let row2 = "Desperados, Turbo King, Amstel Malta";
    let row3 = "Goldberg, Williams, Malta Gold";
    let row4 = "Gulder, , ";
    let row5 = "Heineken, , ";
    let row6 = "Star, , ";

    let mut file = std::fs::File::create("nigerian_breweries.txt").expect("create failed");
     file.write_all(header.as_bytes()).expect("write failed");
     file.write_all(row1.as_bytes()).expect("write failed");
     file.write_all(row2.as_bytes()).expect("write failed");
     file.write_all(row3.as_bytes()).expect("write failed");
     file.write_all(row4.as_bytes()).expect("write failed");
     file.write_all(row5.as_bytes()).expect("write failed");
     file.write_all(row6.as_bytes()).expect("write failed");
     println!("\nData successfully written to nigerian_breweries.txt" );


    
}
