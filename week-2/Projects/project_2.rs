fn main() {

	//naming my variables after the first letter of each product
	let t:f64 = 2.0*450_000.0;
	let m:f64 = 1500_000.0;
	let hp:f64 = 3.0*750_000.0;
	let d:f64 = 3.0*2850_000.0;
	let a:f64 = 250_000.0;
	let qty:f64 = 2.0+1.0+3.0+3.0+1.0;
	println!("The total quantity of goods sold is {}", qty);

	//doing calculations
	let sum:f64 = t+m+hp+d+a;
	println!("The sum of the sales amount is ${}", sum);
	let ave = sum/qty;
	println!("The total average sales is ${}", ave);

}