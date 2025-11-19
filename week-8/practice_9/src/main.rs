fn main() {

    let b:(u8,bool,f32) = (110,true,10.9);
    print(b);
}

//pass the tuple as a parameter

fn print (x:(u8,bool,f32)) {

    println!("\nInside print method");
    println!("{:?}",x);
}