use crate::{floating_decimal::FloatingPointDecimal, transaction::Transaction};

/// Describes the status of an account
pub struct AccountStatus {
	account: String,

	balance: FloatingPointDecimal,
	ingress: FloatingPointDecimal,
	egress: FloatingPointDecimal,
	debt: FloatingPointDecimal,
	pending_pay: FloatingPointDecimal
}

impl AccountStatus {
	/// Creates an account status in zeros
	pub fn new(account: &String) -> Self {
		let zero = FloatingPointDecimal::new(0, 0);

		Self {
			account: account.clone(),

			balance: zero,
			ingress: zero,
			egress: zero,
			debt: zero,
			pending_pay: zero
		}
	}

	/// Updates the account status given a new transaction
	pub fn update(&mut self, transaction: &Transaction) {
		let amount = transaction.get_money();

		if !transaction.is_a_promise() {
			// Ingress
			if transaction.get_to()==self.account {
				self.ingress += amount;
				self.balance += amount;

				self.pending_pay -= amount;
			}

			// Egress
			if transaction.get_from()==self.account {
				self.egress += amount;
				self.balance -= amount;

				self.debt -= amount;
			}
		} else {
			// Pending Payment
			if transaction.get_to()==self.account {
				self.pending_pay += amount;
			}

			// Debt
			if transaction.get_from()==self.account {
				self.debt += amount;
			}
		}
	}

	/// Displays the account status in the CLI
	pub fn show_acc_status(&self) {
		println!("BALANCE: {}", self.balance);
		println!();
		println!("INGRESS: {}", self.ingress);
		println!("EGRESS: {}", self.egress);
		println!();
		println!("DEBT: {}", self.debt);
		println!("PENDING PAYMENT: {}", self.pending_pay);
		println!();
	}

	#[allow(dead_code)]
	/// Returns the balance
	pub fn get_balance(&self) -> FloatingPointDecimal {
		self.balance
	}

	#[allow(dead_code)]
	/// Returns the ingress
	pub fn get_ingress(&self) -> FloatingPointDecimal {
		self.ingress
	}

	#[allow(dead_code)]
	/// Returns the egress
	pub fn get_egress(&self) -> FloatingPointDecimal {
		self.egress
	}

	#[allow(dead_code)]
	/// Returns the debt
	pub fn get_debt(&self) -> FloatingPointDecimal {
		self.debt
	}

	#[allow(dead_code)]
	/// Returns the pending_pay
	pub fn get_pending_pay(&self) -> FloatingPointDecimal {
		self.pending_pay
	}
}