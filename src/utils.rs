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
}