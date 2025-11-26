use std::io::Write;
use std::fs::OpenOptions;
use std::io;

fn main() {

    loop {
        println!("\nEnter the message you want to append:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("This is not a string");

    println!("\nIs there any other message you want to append?(yes/no)");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("This is not a string");
    let choice = input2.trim().to_lowercase();

    let input1 = input1.replace("\\n", "\n");

    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    file.write_all(input1.as_bytes()).expect("write failed");

    if choice == "yes" {
        continue;
    } else if choice == "no"{
        break;
    } else {
        println!("\nPlease input yes/no. - Last chance");
        let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("This is not a string");
    let choice2 = input3.trim().to_lowercase();

    if choice2 == "yes" {
        continue;
    } else if choice2 == "no"{
        break;
    } else {
        break;
    }

    }

    }
     println!("\nFile append success.");

}
