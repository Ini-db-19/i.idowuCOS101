fn main() {
    // Initialize a mutable tuple
    let mut mountain_heights = ("Everest", 8848, "Cow", 7743);

    println!("\nOriginal tuple = {:?}", mountain_heights);

    // change 3rd and 4th element of a mutable tuple
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8829;

    println!("Changed tuple = {:?}", mountain_heights);
}
