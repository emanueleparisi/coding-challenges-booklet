const MAX_FACTORIAL: u64 = 8;


fn solution_recursive(n: u64) -> u64{
	if n == 0 || n == 1 {
		1
	} else {
		n * solution_recursive(n-1)
	}
}


fn solution_iterative(n: u64) -> u64{
	let mut res: u64 = 1;

	for p in 2..n+1 {
		res *= p;
	}
	
	res
}


fn main() {
	println!("===========================================");
	println!("Coding Challenges Booklet - Excercise 01   ");
	println!("Factorial Finder                           ");
	println!("===========================================");

	println!("Recursive solution");
	for n in 0..MAX_FACTORIAL {
		println!(
			"{}! = {}",
			n,
			solution_recursive(n)
		);
	}

	println!("Iterative solution");
	for n in 0..MAX_FACTORIAL {
		println!(
			"{}! = {}",
			n,
			solution_iterative(n)
		);
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution_recursive() {
		assert_eq!(
			solution_recursive(0),
			1
		);
		assert_eq!(
			solution_recursive(1),
			1
		);
		assert_eq!(
			solution_recursive(5),
			120
		);
	}

	#[test]
	fn test_solution_iterative() {
		assert_eq!(
			solution_iterative(0),
			1
		);
		assert_eq!(
			solution_iterative(1),
			1
		);
		assert_eq!(
			solution_iterative(5),
			120
		);
	}
}