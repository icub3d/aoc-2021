struct Decoder {
	v: Vec<usize>,
	pos: usize,
}



fn main() {
	// Collect all the bits and ones and zeros
	let input = include_str!("../../input").lines().next().unwrap();
	let bits = input.chars().flat_map(move |c| {
		// Convert to decimal and turn into it's constituent bits. I'm
		// thinking this will be the best representation because we
		// are pulling non-standard groups of bits at a time.
		let h = c.to_digit(16).unwrap(); 
		format!("{:b}", h).chars().map(|c| {c.to_digit(10).unwrap() as usize}).collect::<Vec<usize>>()
	}).collect::<Vec<usize>>();


	
}
