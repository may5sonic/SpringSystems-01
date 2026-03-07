#[derive(Debug)]
pub struct BankAccount {
    // fields in Rust are private by default unless marked with pub
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // ignore the operation if the amount is negative
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Ignore if the amount is greater than the balance or negative
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    // apply interest
    pub fn apply_interest(&mut self, rate: f64) {
        if rate > 0.0 {
            self.balance += self.balance * rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //function for floating-point comparisons
    fn is_close(a: f64, b: f64) -> bool {
        let epsilon = 1e-10;
        (a - b).abs() < epsilon
    }

    #[test]
    fn test_new_account() {
        let account = BankAccount::new(100.0);
        assert!(is_close(account.balance(), 100.0));
    }

    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert!(is_close(account.balance(), 150.0));
    }

    #[test]
    fn test_deposit_negative_amount() {
        let mut account = BankAccount::new(100.0);
        account.deposit(-50.0); // Should be ignored
        assert!(is_close(account.balance(), 100.0));
    }

    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(40.0);
        assert!(is_close(account.balance(), 60.0));
    }

    #[test]
    fn test_withdraw_more_than_balance() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(150.0); // Should be ignored
        assert!(is_close(account.balance(), 100.0));
    }

    #[test]
    fn test_withdraw_negative_amount() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(-20.0); // Should be ignored
        assert!(is_close(account.balance(), 100.0));
    }

    #[test]
    fn test_apply_interest() {
        let mut account = BankAccount::new(100.0);
        account.apply_interest(0.05); // 5% interest
        assert!(is_close(account.balance(), 105.0));
    }
}