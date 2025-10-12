fn main (){
	let a:f64 = 450_000.0 * 2.0;
	let x:f64 = 1_500_000.0;
	let r:f64 = 2_850_000.0 * 3.0;
	let n:f64 = 750_000.0 * 3.0;
	let f:f64 = 250_000.0;

	let s = a + x + r + n + f; // Sum
	println! ("Sum is {:.2}",s);
	let avg = s/10.0; // Average
	println!("Average is {:.2}",avg );
}
	
