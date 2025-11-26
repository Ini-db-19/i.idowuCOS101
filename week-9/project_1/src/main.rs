use std::io::Write;

fn main() {
 
    let table = "Lager           Stout         Non-Alcoholic
    \n33 Export       Legend              Maltina
    \nDesperados    Turbo King       Amstel Malta
    \nGoldberg       Williams          Malta Gold
    \nGulder                              Fayrouz
    \nHeineken
    \nStar";
    
    let mut file = std::fs::File::create("Nigerian Brewery Limited Portfolio.txt").expect("Create failed");
    file.write_all(table.as_bytes()).expect("Write failed");

    println!("\nFile successfully created.");
}

