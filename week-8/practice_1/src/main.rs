fn main() {

    // Using Vec::new()
    let v : Vec<i64> = Vec::new();

    //printing the vector's size
    println!("\nThe length of the Vector is {}", v.len());

    //using macro
    let v = vec!["Bob","Jake","Basiduke","Jason","Roberto"];

    //printing the vector's size
    println!("\nThe length of the vec macro is: {}",v.len());

}
