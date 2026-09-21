fn main() {
let n = 42;
println!("Is {} even? {}", n, is_even(n));

}
 
	fn is_even(n: i32) -> bool { 
n % 2 == 0
}
