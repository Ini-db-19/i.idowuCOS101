fn main() {

    let v = vec![20,23,34,33,44];
    print_vector(v);
    println!("{}",v[0]);
}

fn print_vector(x:Vec<i32>){
    println!("Inside print_vector function{:?}", x);
}