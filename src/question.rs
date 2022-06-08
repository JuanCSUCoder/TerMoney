use std::io::stdin;

/// It is a class that handle validation of questions in the terminal
pub struct Question {
	message: String,
	invalids: Vec<String>
}

impl Question {
	/// Creates a new instance of the question
	pub fn new(msg: &str) -> Self {
		
		Self {
			message: msg.to_string(),
			invalids: Vec::new()
		}
	}

	/// Sets the question configuration to refuse null answers
	pub fn not_null(&mut self) -> &mut Self {
		self.invalids.push("".to_string());

		self
	}

	/// Method description
	pub fn not_valid(&mut self, value: String) -> &mut Self {
		self.invalids.push(value);

		self
	}

	/// Asks the configured question and returns a value
	pub fn ask(&self) -> String {
		let mut answer = String::new();
		let mut answered = false;

		while !answered {
			println!("{}", self.message);

			stdin().read_line(&mut answer).expect("Error al leer la respuesta");
			answer = answer.trim().to_string();

			answered = true;
			for invalid in &self.invalids {
				if answer==*invalid {
						answered = false;
				}
			}
		}

		answer
	}
}