use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


const SPEED_LIMIT    : f64 = 70.0;


#[derive(Debug)]
struct Record {
	id: String,
	t0: f64,
	t1: f64,
	v : f64
}


impl Record {
	const CAMERA_DISTANCE: f64 = 1.0;

	fn from_string(s: &str) -> Result<Record, &'static str> {
		let parse_id = |x: &str| -> Result<String, &'static str> {
			let valid = x.chars().enumerate().all(|(i, c)| {
				match i {
					0..=1 => c.is_alphabetic(),
					2..=6 => c.is_numeric(),
					_     => false
				}
			});

			if valid {
				Ok(String::from(x))
			} else {
				Err("Invalid ID format")
			}
		};
		
		let fields: Vec<&str> = s.split(' ').collect();

		if fields.len() != 3 {
			return Err("Expected 3 fields");
		}

		let id = parse_id(fields[0]);
		let t0 = fields[1].parse::<f64>();
		let t1 = fields[2].parse::<f64>();

		match (id, t0, t1) {
			(Ok(id) , Ok(t0) , Ok(t1) ) => Ok(Record{
				id: id, 
				t0: t0,
				t1: t1,
				v : Self::CAMERA_DISTANCE/(t1-t0)
			}),
			(Err(..), _      , _      ) => Err("Invalid plate number"),
			(_      , Err(..), _      ) => Err("Invalid time0"),
			(_      , _      , Err(..)) => Err("Invalid time1")
		}
	}
}


fn main() {
	println!("===========================================");
	println!("Coding Challenges Booklet - Excercise 02   ");
	println!("Speed Tracker                              ");
	println!("===========================================");

	let path   = Path::new("src/02_speed_tracker/input.txt");
	let file   = File::open(path).expect("File does not exist!");
	let reader = BufReader::new(file);

	for line in reader.lines() {
		match line {
			Ok(l) => match Record::from_string(&l) {
				Ok(record) => {
					if record.v > SPEED_LIMIT {
						println!("{l:?}");
						println!("\tValid - Too fast!")
					} else {
						println!("{l:?}");
						println!("\tValid - Speed correct.")
					}
				},
				Err(msg) => {
					println!("{l:?}");
						println!("\tInvalid - {msg}")
				}
			},
			Err(e) => println!("{e}")
		};
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_string00() -> Result<(), String>{
		match Record::from_string("AA00000 0.0 0.1") {
			Ok(..) => Ok(()),
			_      => Err(String::from("Valid line return Err()"))
		}
	}

	#[test]
	fn test_from_string01() -> Result<(), String>{
		match Record::from_string("A000000 0.0 0.1") {
			Ok(..) => Err(String::from("Invalid line return Ok()")),
			_      => Ok(())
		}
	}

	#[test]
	fn test_from_string02() -> Result<(), String>{
		match Record::from_string("AAA0000 0.0 0.1") {
			Ok(..) => Err(String::from("Invalid line return Ok()")),
			_      => Ok(())
		}
	}

	#[test]
	fn test_from_string03() -> Result<(), String>{
		match Record::from_string("A/00000 0.0 0.1") {
			Ok(..) => Err(String::from("Invalid line return Ok()")),
			_      => Ok(())
		}
	}

	#[test]
	fn test_from_string04() -> Result<(), String>{
		match Record::from_string("AA000000 0.0 0.1") {
			Ok(..) => Err(String::from("Invalid line return Ok()")),
			_      => Ok(())
		}
	}

	#[test]
	fn test_from_string05() -> Result<(), String>{
		match Record::from_string("AA000000 0.x 0.1") {
			Ok(..) => Err(String::from("Invalid line return Ok()")),
			_      => Ok(())
		}
	}
}