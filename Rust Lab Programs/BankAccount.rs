struct BankAccount {
    account_number: i32,
    holder_name: String,
    balance: f64,
}

impl BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ${:.2}. New balance: ${:.2}", amount, self.balance);
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            println!("Insufficient balance.");
        } else {
            self.balance -= amount;
            println!("Withdrawn ${:.2}. New balance: ${:.2}", amount, self.balance);
        }
    }

    fn display_details(&self) {
        println!("Account Number: {}", self.account_number);
        println!("Holder Name: {}", self.holder_name);
        println!("Balance: ${:.2}", self.balance);
    }
}

fn main() {
    let mut account = BankAccount {
        account_number: 12345,
        holder_name: "John Doe".to_string(),
        balance: 1000.0,
    };

    account.display_details();
    account.deposit(500.0);
    account.withdraw(200.0);
    account.display_details();
}