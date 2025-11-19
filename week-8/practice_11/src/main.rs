fn main() {

    //an array of numbers
    let numbers = [1,2,3,4,5];
    println!("\nOriginal array = {:?}", numbers);

    //create a slice of second and third element
    let slice1 = &numbers[1..3];
    println!("Second and third elements sliced = {:?}",slice1);    

    //Omit the start index
    let slice2 = &numbers[..3];
    //The slice starts from index 0 to index 3
    println!("Index 0 to 3 sliced = {:?}",slice2); 

    //omit the end index
    let slice3 = &numbers[2..];
    //This means the slice starts from index 2 to 5
    println!("Index 2 to index 5 sliced = {:?}",slice3); 

    /*omit the start and end index
    then reference the whole array*/
    let slice4 = &numbers[..];
    //This means the slice starts from index 0 to 5
    println!("Index 0 to 5 sliced = {:?}",slice4); 
}
