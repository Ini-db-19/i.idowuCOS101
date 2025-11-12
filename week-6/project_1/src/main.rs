 bfn main() {

    use std::io;

    println!("\n==================== Food Menu =========================
        \nItem Code             Product                      Price 
        \nP            Poundo Yam/Edinkaiko Soup            N3_200 
        \nF             Fried Rice & chicken                N3_000 
        \nA              Amala & Ewedu Soup                 N2_500 
        \nE                Eba & Egusi Soup                 N2_000
        \nW               White rice & Stew                 N2_500");

    //Making inputs changeable
    let mut input1 = String::new();
    let mut input2 = String::new();

    //Input for item code
    println!("\nInput your desired product's item code: ");
    io::stdin().read_line(&mut input1).expect("This is not a valid string");
    let code = input1.trim().to_lowercase();

    //Input for quantity of products
    println!("\nHow many do you want?");
    io::stdin().read_line(&mut input2).expect("This is not a valid string");
    let quantity:f32 = input2.trim().parse().expect("This is not a real integer");

    //Matching food type to price
    let price = match code.as_str() {
        "p" => 3_200.00,
        "f" => 3_000.00,
        "a" => 2_500.00,
        "e" => 2_000.00,
        "w" => 2_500.00,

         _ => {
        println!("\nPlease input a valid item code.");
        return;
    } 

    };

    //Defining both total and discounted prices
    let total = price * quantity;
    let discount = total * 0.95;

    //Apply discount if total price is more than N10 000.
    if total > 10_000.00 {
        println!("\nThe total cost of your order is: N{:.2}.", discount);
    }

    //Print normal total if price is N10 000 or less.
     else {
        println!("\nThe total cost of yout order is: N{}.", total);
    }
}
