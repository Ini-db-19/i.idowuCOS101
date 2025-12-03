struct Employee{
    ceo_name:String,
    company:String,
    age:u32
}

fn main() {
    //Initializing a structure
    let emp1 = Employee{
        company:String::from("Microsoft Corporation"),
        ceo_name:String::from("Satya Nadella"),
        age:56
    };
    let emp2 = Employee{
        company:String::from("Google Inc."),
        ceo_name:String::from("Sundai Pichai"),
        age:51
    };
    //passing emp1 and emp2 to the display() function
    display(emp1);
    display(emp2);
}

//Get the values of the specific structs and print it to the console
fn display(emp:Employee){
    println!("\nName is : {}, company is : {}, age is : {}.", emp.ceo_name,emp.company,emp.age);
}
