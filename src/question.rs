use std::{io::{stdin, stdout, Write}, u64};

/// It is a class that handle validation of questions in the terminal
pub struct Question {
	message: String,
	invalids: Vec<String>,
	invalids_contents: Vec<String>
}

impl Question {
	/// Creates a new instance of the question
	pub fn new(msg: &str) -> Self {
		
		Self {
			message: msg.to_string(),
			invalids: Vec::new(),
			invalids_contents: Vec::new()
		}
	}

	/// Sets the question configuration to refuse null answers
	pub fn not_null(&mut self) -> &mut Self {
		self.invalids.push("".to_string());

		self
	}

	/// Adds an invalid answer
	pub fn not_valid(&mut self, value: &str) -> &mut Self {
		self.invalids.push(value.to_string());

		self
	}

	/// Adds a restriction for the answer
	pub fn not_containing(&mut self, value: &str) -> &mut Self {
		self.invalids_contents.push(value.to_string());

		self
	}

	/// Asks the configured question and returns a value
	pub fn ask(&self) -> String {
		let mut answer = String::new();
		let mut answered = false;

		while !answered {
			print!("{}", self.message);
			stdout().flush().expect("Error flushing console output");
			answer = String::new();

			stdin().read_line(&mut answer).expect("Error reading the answer");
			answer = answer.trim().to_string();

			answered = true;

			for invalid in &self.invalids {
				if answer==*invalid {
					answered = false;
				}
			}

			for inv_cont in &self.invalids_contents {
				if answer.contains(inv_cont) {
					answered = false;

					println!("Cannot have \"{}\" in the answer!", inv_cont);
				}
			}

			if !answered {
				println!("Invalid answer \"{}\"!", answer);
			}
		}

		answer
	}

	/// Asks the configured question and resturns a number
	pub fn ask_number(&self) -> i64 {
		let mut number: i64 = 0;
		let mut is_valid = false;

		while !is_valid {
			match self.ask().parse::<i64>() {
				Ok(num) => {
					number = num;
					is_valid = true;
				}
				Err(_) => println!("Invalid number!")
			}
		}

		number
	}

	/// Asks the configured question and returns an unsigned number
	pub fn ask_positive(&self) -> u64 {
		let mut number: u64 = 0;
		let mut is_valid = false;

		while !is_valid {
			match u64::try_from(self.ask_number()) {
				Ok(result) => {
					number = result;
					is_valid = true;
				},
				Err(_) => println!("Must be a positive number")
			}
		}

		number
	}

	/// Asks the configured yes or no question and return the result as a boolean
	pub fn ask_yn(&self) -> bool {
		let mut answer = false;
		let mut valid = false;

		while !valid {
			valid = true;

			match &self.ask().to_uppercase()[..] {
				"Y" => answer = true,
				"N" => answer = false,
				"YES" => answer = true,
				"NO" => answer =false,

				_ => {
					valid = false;

					println!("Please select YES(Y/y) or NO(N/n)");
				}
			}
		}

		answer
	}
}