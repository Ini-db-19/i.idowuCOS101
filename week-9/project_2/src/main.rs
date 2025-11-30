use std::io::Write;

fn main() {
    let head ="\n                         PAU SMIS";
    let columns ="\nStudent Name       Matric. Number    Department      Level";
    let oluchi = ["Oluchi Mordi","ACC10211111","Accounting","300"];
    let amilah = ["Amilah Ziga","ECO10110101","Economics","200"];
    let ali = ["Muhammad Ali","CSC10328828","Computing","100"];
    let bob = ["Adekunle Bobby","EEE11020202","Electrical","300"];
    let peter = ["Bianca Peterson","MEE10202001","Mechanical","100"];

    println!("{}", head);
    println!("{}", columns);

    println!("\n{}        {}      {}       {}", oluchi[0], oluchi[1], oluchi[2], oluchi[3]);
    println!("{}         {}      {}        {}", amilah[0], amilah[1], amilah[2], amilah[3]);
    println!("{}        {}      {}        {}", ali[0], ali[1], ali[2], ali[3]);
    println!("{}      {}      {}       {}", bob[0], bob[1], bob[2], bob[3]);
    println!("{}     {}      {}       {}", peter[0], peter[1], peter[2], peter[3]);

    let line = format!("\n{}        {}      {}       {}", oluchi[0], oluchi[1], oluchi[2], oluchi[3]);
    let line1 = format!("\n{}         {}      {}        {}", amilah[0], amilah[1], amilah[2], amilah[3]);
    let line2 = format!("\n{}        {}      {}        {}", ali[0], ali[1], ali[2], ali[3]);
    let line3 = format!("\n{}      {}      {}       {}", bob[0], bob[1], bob[2], bob[3]);
    let line4 = format!("\n{}     {}      {}       {}", peter[0], peter[1], peter[2], peter[3]);

    let mut file = std::fs::File::create("PAU Student Management Information System.txt").expect("Create failed");
    file.write_all(head.as_bytes()).expect("write failed");
    file.write_all(columns.as_bytes()).expect("write failed");
    file.write_all(line.as_bytes()).expect("write failed");
    file.write_all(line1.as_bytes()).expect("write failed");
    file.write_all(line2.as_bytes()).expect("write failed");
    file.write_all(line3.as_bytes()).expect("write failed");
    file.write_all(line4.as_bytes()).expect("write failed");

    println!("\nFile created successfully.");
}
