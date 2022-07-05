/// A set of utilities
pub struct Utils {}

impl Utils {
	/// Prints a line of the specified lenght
	pub fn print_line(lenght: usize) {
		for _ in 0..lenght {
			print!("-");
		}

		println!();
	}

	/// Returns the string of digits with a thousand separator
	pub fn add_thousand_separator(digits: &String) -> String {
		let mut temp = String::from("");
		let mut counter = 0;
		let mut full_count = 0;

		for digit in digits.chars().rev() {
			temp.push(digit);
			counter += 1;
			full_count += 1;

			if counter==3 && full_count<digits.len() {
				temp.push('.');
				counter = 0;
			}
		}

		temp.chars().rev().collect::<String>()
	}
}