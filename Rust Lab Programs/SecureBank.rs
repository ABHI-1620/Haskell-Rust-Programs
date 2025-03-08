use std::io;

struct BankAccount {
    account_number: String,
    owner_name: String,
    balance: f64,
}

impl BankAccount {
    fn new(account_number: &str, owner_name: &str, balance: f64) -> BankAccount {
        BankAccount {
            account_number: account_number.to_string(),
            owner_name: owner_name.to_string(),
            balance,
        }
    }
    fn view_balance(&self) -> f64 {
        self.balance
    }
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            Err("Insufficient funds!".to_string())
        } else {
            self.balance -= amount;
            Ok(())
        }
    }
    fn display_account_details(&self) {
        println!("\nAccount Details:");
        println!("Account Number: {}", self.account_number);
        println!("Account Owner: {}", self.owner_name);
    }
}

fn main() {

    let mut account_number = String::new();
    let mut owner_name = String::new();
    let mut balance_input = String::new();

    println!("Enter account number: ");
    io::stdin().read_line(&mut account_number).expect("Failed to read line");

    println!("Enter owner's name: ");
    io::stdin().read_line(&mut owner_name).expect("Failed to read line");

    println!("Enter initial balance: ");
    io::stdin().read_line(&mut balance_input).expect("Failed to read line");

    let balance: f64 = balance_input.trim().parse().expect("Please input a valid number");
    let mut account = BankAccount::new(&account_number.trim(), &owner_name.trim(), balance);
    account.display_account_details();
    println!("\nBalance before any transactions: ${:.2}", account.view_balance());
    let mut deposit_input = String::new();
    println!("Enter deposit amount: ");
    io::stdin().read_line(&mut deposit_input).expect("Failed to read line");
    let deposit: f64 = deposit_input.trim().parse().expect("Please input a valid number");
    account.deposit(deposit);
    println!("Balance after deposit: ${:.2}", account.view_balance());
    let mut withdraw_input = String::new();
    println!("Enter withdrawal amount: ");
    io::stdin().read_line(&mut withdraw_input).expect("Failed to read line");
    let withdraw: f64 = withdraw_input.trim().parse().expect("Please input a valid number");
    match account.withdraw(withdraw) {
        Ok(_) => println!("Balance after withdrawal: ${:.2}", account.view_balance()),
        Err(e) => println!("Error: {}", e),
    }
}