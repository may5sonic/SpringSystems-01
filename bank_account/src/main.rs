mod bank_account;
use bank_account::BankAccount;

fn main() {
    println!("--- Bank Account Demo ---");

    // Create a new account
    let mut my_account = BankAccount::new(500.0);
    println!("Initial balance: ${:.2}", my_account.balance());

    // Deposit Money
    my_account.deposit(150.0);
    println!("After depositing $150.00, balance is: ${:.2}", my_account.balance());

    // Withdraw money
    my_account.withdraw(100.0);
    println!("After withdrawing $100.00, balance is: ${:.2}", my_account.balance());

    // Edge case: Withdraw more than balance
    my_account.withdraw(1000.0);
    println!("Attempted to withdraw $1000.00. Current balance: ${:.2}", my_account.balance());

    // Bonus: Apply Interest
    my_account.apply_interest(0.05); // 5% interest
    println!("After applying 5% interest, final balance is: ${:.2}", my_account.balance());
}
