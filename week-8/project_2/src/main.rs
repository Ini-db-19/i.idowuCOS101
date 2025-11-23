fn main() {
    let mut names: Vec<String> = Vec::new();
    let mut experience: Vec<f32> = Vec::new();

    println!("\nHow many developers do you want to input?");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let dev_num: usize = input1.trim().parse().expect("Invalid integer");

    for count in 0..dev_num {
        println!("\nEnter developer name {}", count + 1);
        let mut input2 = String::new();
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let dev_name = input2.trim().to_string();
        names.push(dev_name);

        println!("\nEnter years of experience for {}", names[count]);
        let mut input3 = String::new();
        std::io::stdin().read_line(&mut input3).expect("Failed to read input");
        let experience_years: f32 = input3.trim().parse().expect("Invalid number");
        experience.push(experience_years);
    }

    let mut max_exp = experience[0];
    let mut max_name = &names[0];

    for i in 1..dev_num {
        if experience[i] > max_exp {
            max_exp = experience[i];
            max_name = &names[i];
        }
    }

    println!("\nThe developer with the highest experience is {} with {} years.",max_name, max_exp);
}
