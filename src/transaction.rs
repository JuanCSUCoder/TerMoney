use chrono::{DateTime, Utc};
use prettytable::Table;
use serde::{Deserialize, Serialize};

use crate::floating_decimal::FloatingPointDecimal;

#[derive(Serialize, Deserialize, Clone)]
/// A transaction
pub struct Transaction {
    id: u64,
    continues: Option<u64>,

    time: DateTime<Utc>,

    from: String,
    to: String,
    description: Option<String>,

    money: FloatingPointDecimal,
}

impl Transaction {
    /// Create a new Transaction
    pub fn new(
        id: u64,
        continues: Option<u64>,
        from: String,
        to: String,
        money: FloatingPointDecimal,
        description: Option<String>,
    ) -> Self {
        Self {
            id,
            continues,

            time: Utc::now(),

            from,
            to,
            description,

            money,
        }
    }

    /// Returns the ID of the transaction
    pub fn get_id(&self) -> u64 {
        self.id
    }

		/// Returns the ID of the assosiated promise transaction
		pub fn get_continue(&self) -> Option<u64> {
			self.continues
		}

    /// Returns the from attribute
    pub fn get_from(&self) -> String {
        self.from.clone()
    }

    /// Returns the to attribute
    pub fn get_to(&self) -> String {
        self.to.clone()
    }

		/// Returns the amount of the transaction
		pub fn get_money(&self) -> FloatingPointDecimal {
			self.money
		}

		/// Returns the description of the transaction
		pub fn get_desc(&self) -> Option<String> {
			self.description.clone()
		}

		/// Returns if the transaction is a promise or not
		pub fn is_a_promise(&self) -> bool {
			self.continues==None
		}

    /// Prints the transaction in a table row
    pub fn print_row(&self, table: &mut Table) {
        let kind = match self.continues {
            None => "Promise".to_string(),
            Some(cod) => format!("Pay-{}", cod),
        };

        let desc = match &self.description {
            Some(val) => val.clone(),
            None => "  -  ".to_string(),
        };

        let desc = desc
            .chars()
            .collect::<Vec<char>>()
            .chunks(33)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        table.add_row(row![FB -> self.id, self.time.format("%a %b %e/%y"), desc, self.from, self.to, kind, r -> self.money]);
    }

		/// Prints the transaction in a table row from an account perspective
		pub fn print_row_perspective(&self, table: &mut Table, account: &String) {
			// Prepare printables
			let kind = match self.continues {
				None => "Promise".to_string(),
				Some(cod) => format!("Pay-{}", cod),
			};

			let desc = match &self.description {
				Some(val) => val.clone(),
				None => "  -  ".to_string(),
			};

			let desc = desc
				.chars()
				.collect::<Vec<char>>()
				.chunks(33)
				.map(|c| c.iter().collect::<String>())
				.collect::<Vec<String>>()
				.join("\n");

			// Ingress
			if account==&self.to {
				table.add_row(row![FB -> self.id, self.time.format("%a %b %e/%y"), desc, kind, self.from, g -> self.money]);
			}
			
			// Egress
			if account==&self.from {
				table.add_row(row![FB -> self.id, self.time.format("%a %b %e/%y"), desc, kind, self.to, r -> format!("-{}", self.money)]);
			}
		}

	}
