use crate::{question::Question, utils::Utils};

/// A CLi Menu
pub struct Menu {
	title: String,
	options: Vec<String>,
	exit: bool,
}

impl Menu {
	/// Creates a new menu
	pub fn new(title: &str) -> Self {
		let title = title.to_string();
		
		Self {
			title,
			options: Vec::new(),
			exit: true,
		}
	}

	/// Adds an option to the menu
	pub fn add_option(mut self, option: &str) -> Self {
		self.options.push(option.to_string());

		self
	}

	#[allow(dead_code)]
	/// Prevents the menu from generating an exit option
	pub fn not_exit(&mut self) {
		self.exit = false;
	}

	/// Displays the configured menu
	pub fn display(&self) -> usize {
		println!();
		println!("{}", self.title);
		Utils::print_line(self.title.len());

		let mut choosen: usize;

		loop {
			let mut counter: u8 = 1;
			for option in &self.options {
				println!("{}. {}", counter, option);

				counter = counter + 1;
			}

			println!("0. Exit");
			Utils::print_line(self.title.len());
			println!();

			choosen = Question::new(" > ").ask_positive().try_into().unwrap();

			if choosen<=self.options.len() && (choosen!=0 || self.exit) {
				break;
			}
		}

		choosen
	}
}