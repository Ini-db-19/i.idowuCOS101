struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle {
    fn area(&self)->u32{

        self.width*self.height  // use the . operator to fetch the value of a field using 'self'
    }
}
fn main() {
    //initiate the struct
    let small = Rectangle{
        width:10,
        height:20
    };
    //print the rectangle's area
    println!("\nWidth is {}. \nHeight is {}. \nArea of Rectangle is {}.",small.width,small.height,small.area());
}
