struct Employee{
    name:String,
    company:String,
    age:u32
}

fn main(){
    let emp1 = Employee{
        company:String::from("Ernst & Young"),
        name:String::from("Jones Jessica"),
        age:30
    };
    println!("\nName = {} \n", emp1.name);
    println!("Company = {} \n", emp1.company);
    println!("Age = {} \n", emp1.age);
}