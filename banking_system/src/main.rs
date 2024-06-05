#![deny(clippy::all)]

trait Account {
    //deposit, withdraw, and balance
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&mut self) -> f64; //view
}

struct BankAccount {
    account_number: i32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Insufficient funds!");
        }
    }

    fn balance(&mut self) -> f64 {
        self.balance
    }
}
fn main() {
    let mut user1 = BankAccount {
        account_number: 123,
        holder_name: "Jumana".to_string(),
        balance: 3780.4,
    };

    let mut user2 = BankAccount {
        account_number: 321,
        holder_name: "Sarah".to_string(),
        balance: 9100.1,
    };

    user1.deposit(22.1);
    user2.withdraw(900.0);

    println!("Jumana's balance= {}", user1.balance());
    println!("Sarah's balance= {}", user2.balance());
}
