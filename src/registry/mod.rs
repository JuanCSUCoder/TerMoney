mod transaction;

use self::transaction::Transaction;

/// A transaction registry
pub struct Registry {
	new_id: i64,
	accounts: Vec<String>,
	transactions: Vec<Transaction>,
}
