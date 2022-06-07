use std::io::stdin;

/// It is a class that handle validation of questions in the terminal
pub struct Question {
	message: String,
	null: bool
}

impl Question {
	/// Creates a new instance of the question
	pub fn new(msg: &str) -> Self {
		
		Self {
			message: msg.to_string(),
			null: true
		}
	}

	/// Sets the question configuration to refuse null answers
	pub fn not_null(&mut self) -> &mut Self {
		self.null = false;

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

			if self.null || answer!="" {
				answered = true;
			}
		}

		answer
	}
}