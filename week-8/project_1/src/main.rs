fn user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("This is not a string");
    let input = input.trim().to_lowercase();
    input
}


fn main() {
    println!("\nPublic Service APS level checker");
    
    println!("\nHello!");
    let name = user_input("\nWhat is your name?");
    let occupation = user_input("\nWhat is your occupation?");

    println!("\nHow many years of work experience do you have?");
    let mut work = String::new();
    std::io::stdin().read_line(&mut work).expect("This is not a string");
    let experience:f32 = work.trim().parse().expect("This is not a valid integer");

    let office_admin = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    let academic = vec!["-","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    let lawyer = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
    let teacher = vec!["Placement","Classroom Teacher","Snr Teacher","Leading Teacher","Deputy Principal","Principal"];
    let levels = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];

   if occupation == "lawyer" &&  experience >= 1.0 && experience < 3.0 {
                println!("\n{}, you are a {} at {} position.", name, lawyer[0], levels[0]);
                }
                else if occupation == "lawyer" &&  experience >= 3.0 && experience <= 5.0 {
                    println!("\n{}, you are a {} at {} position.", name, lawyer[1], levels[1]);
                }
                else if occupation == "lawyer" &&  experience > 5.0 && experience <= 8.0 {
                    println!("\n{}, you are a {} at {} position.", name, lawyer[2], levels[2]);
                }
                else if occupation == "lawyer" &&  experience > 8.0 && experience <= 10.0 {
                    println!("\n{}, you are a {} at {} position.", name, lawyer[3], levels[3]);
                }
               else if occupation == "lawyer" &&  experience > 10.0 && experience <= 13.0 {
                    println!("\n{}, you are a {} at {} position.", name, lawyer[4], levels[4]);
                } 
                else if occupation == "lawyer" &&  experience > 13.0 {
                    println!("\n{}, you are a {} at {} position.", name, lawyer[5], levels[5]);
                } 
                 else if occupation == "teacher" &&  experience >= 1.0 && experience < 3.0 {
                    println!("\n{}, you are a {} at {} position.", name, teacher[0], levels[0]);
                }
                else if occupation == "teacher" &&  experience >= 3.0 && experience <= 5.0 {
                    println!("\n{}, you are a {} at {} position.", name, teacher[1], levels[1]);
                }
                else if occupation == "teacher" &&  experience > 5.0 && experience <= 8.0 {
                    println!("\n{}, you are a {} at {} position.", name, teacher[2], levels[2]);
                }
                else if occupation == "teacher" &&  experience > 8.0 && experience <= 10.0 {
                    println!("\n{}, you are a {} at {} position.", name, teacher[3], levels[3]);
                }
               else if occupation == "teacher" &&  experience > 10.0 && experience <= 13.0 {
                    println!("\n{}, you are a {} at {} position.", name, teacher[4], levels[4]);
                } 
                else if occupation == "teacher" &&  experience > 13.0 {
                    println!("\n{}, you are a {} at {} position.", name, teacher[5], levels[5]);
                } 
                else if occupation == "academic" &&  experience >= 1.0 && experience < 3.0 {
                    println!("\n{}, you are a {} at {} position.", name, academic[0], levels[0]);
                }
                else if occupation == "academic" &&  experience >= 3.0 && experience <= 5.0 {
                    println!("\n{}, you are a {} at {} position.", name, academic[1], levels[1]);
                }
                else if occupation == "academic" &&  experience > 5.0 && experience <= 8.0 {
                    println!("\n{}, you are a {} at {} position.", name, academic[2], levels[2]);
                }
                else if occupation == "academic" &&  experience > 8.0 && experience <= 10.0 {
                    println!("\n{}, you are a {} at {} position.", name, academic[3], levels[3]);
                }
               else if occupation == "academic" &&  experience > 10.0 && experience <= 13.0 {
                    println!("\n{}, you are a {} at {} position.", name, academic[4], levels[4]);
                } 
                else if occupation == "academic" &&  experience > 13.0 {
                    println!("\n{}, you are a {} at {} position.", name, academic[5], levels[5]);
                } 
                else if occupation == "office administrator" &&  experience >= 1.0 && experience < 3.0 {
                    println!("\n{}, you are a {} at {} position.", name, office_admin[0], levels[0]);
                }
                else if occupation == "office administrator" &&  experience >= 3.0 && experience <= 5.0 {
                    println!("\n{}, you are a {} at {} position.", name, office_admin[1], levels[1]);
                }
                else if occupation == "office administrator" &&  experience > 5.0 && experience <= 8.0 {
                    println!("\n{}, you are a {} at {} position.", name, office_admin[2], levels[2]);
                }
                else if occupation == "office administrator" &&  experience > 8.0 && experience <= 10.0 {
                    println!("\n{}, you are a {} at {} position.", name, office_admin[3], levels[3]);
                }
               else if occupation == "office administrator" &&  experience > 10.0 && experience <= 13.0 {
                    println!("\n{}, you are a {} at {} position.", name, office_admin[4], levels[4]);
                } 
                else if occupation == "office administrator" &&  experience > 13.0 {
                    println!("\n{}, you are a {} at {} position.", name, office_admin[5], levels[5]);
                } 
                else {
                    println!("\nKindly input a proper occupation or work for more than a year beforehand.");
                }


}
