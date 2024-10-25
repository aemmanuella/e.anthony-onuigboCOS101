fn main(){
	// individual sales amount
	let toshiba = 450_000.0;
	let mac = 1_500_000.0;
	let hp = 750_000.0;
	let dell = 2_850_000.0;
	let acer = 250_000.0;

	//sum of amounts
	let sum = toshiba + mac + hp + dell + acer;

	//average of amounts
	let average = sum / 5.0;
	println!("The sum of the amounts is {}", sum);
	println!("The average of the amounts is {}", average);
}