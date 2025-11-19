fn main() {

    let b:(i32,bool,f32) = (30,true,4.9);
    print(b);
}

fn print(x:(i32,bool,f32)) {

    println!("\nInside print method");
    //assigns a tuple to distinct variables
    let (age,is_male,cgpa) = x;
    println!("\nAge is {}, Male? {}, Cgpa is {}",age,is_male,cgpa );
}