
struct Price {
    hp:u64,ibm:u64,toshiba:u64,dell:u64
}

impl Price {
   fn total(&self) -> u64 {
        3*(self.hp+self.ibm+self.toshiba+self.dell)
    }
}

fn main() {

let price = Price{
    hp:650_000,ibm:755_000,toshiba:550_000,dell:850_000
};
    println!("\nDear customer, you have ordered three HP laptops, three IBM laptops, three Toshiba laptops and three DELL laptops. Your total cost is {}.", price.total());
}

