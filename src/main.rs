fn main() {
let n = 42;
println!("Is {} even? {}", n, is_even(n));

	let num = 12345;
	println!("Sum of digits: {}", sum_digits(num));
}
 
	fn is_even(n: i32) -> bool { 
n % 2 == 0
}

fn next() {
}

fn sum_digits(num: i32) -> u32 {
	let mut current = num.unsigned_abs();
	let mut sum: u32 = 0;

	while current > 0 {
		sum += current % 10;
		current /= 10;
	}

	sum
}



