
struct Price {
    hp:u32,ibm:u32,toshiba:u32,dell:u32
}

impl Price {
   fn total(&self) -> u32 {
        3*(self.hp+self.ibm+self.toshiba+self.dell)
    }
}

fn main() {

let price = Price{
    hp:650_000,ibm:755_000,toshiba:550_000,dell:850_000
};
    println!("\nDear customer, you have ordered three HP laptops, three IBM laptops, three Toshiba laptops and three DELL laptops. Your total cost is {}.", price.total());
}

