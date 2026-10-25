use std::io::Write;

fn main() {
    let header = r"Student Name\Matric Number\Department\Level";

    let students = vec![
        ("Oluchi Mordi", "ACC10211111",  "Accounting", "300"),
        ("Adams Aliyu",  "ECO10110101",  "Economics",  "100"),
        ("Shania Bolade","CSC10328828",  "Computer",   "200"),
        ("Adekunle Gold","EEE11020202",  "Electrical", "200"),
        ("Blanca Edemoh","MEE10202001",  "Mechanical", "100"),
];
    println!("{}", header);
    println!();
    for s in students {
    println!("{},{},{},{}", s.0, s.1, s.2, s.3);
    }

    let mut file = std::fs::File::create("students.txt ").expect("create failed ");
    file.write_all(header.as_bytes()).expect("write failed");
    println!("\n Data written successfully to students.txt");


}
