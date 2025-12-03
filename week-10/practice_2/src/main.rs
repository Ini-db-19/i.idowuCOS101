fn main() {

    let v = vec![10,22,45];

    let v2 = v;

    display(v2);
    //v2 is moved to the display function and v2 is invalidated past here.

    println!("In main {:?}", v2);
    //v2 has been transferred so it can't be used here
}

fn display(v: Vec<i32>){
    println!("Inside display {:?}", v);
}
