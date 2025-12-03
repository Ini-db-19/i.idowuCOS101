fn main() {

    let v = vec![12,22,34,55];

    let v2 = v;
    let v2_return = display(v2);
    println!("In main {:?}",v);
}

fn display(v:Vec<i32>) -> Vec<i32> {
    //returning the vector
    println!("Inside display {:?}",v);
    v
}