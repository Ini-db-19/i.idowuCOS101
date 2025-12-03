fn main() {

    let x = vec![100,222,444];
    borrow_vector(&x); //Passing by reference

    println!("Printing the value from main() x[0]={}",x[0]);
    println!("*****************************");
}

fn borrow_vector(z:&Vec<i32>){

    println!("*****************************");
    println!("Inside borrow_vector function {:?} \n",z);
    println!("-----------------------------");
}
