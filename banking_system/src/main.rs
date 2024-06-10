#![deny(clippy::all)]

trait Account {
    //deposit, withdraw, and balance
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&mut self) -> f64; //view
}

struct BankAccount {
    account_number: i32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount < 0.0 {
            return Err("Deposit amount must be positive".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= self.balance {
            self.balance -= amount;
            return Ok(())
        } else {
           return Err("Insufficient funds!".to_string());
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

    match user1.deposit(22.1){
        Ok(())=>println!("Deposit successful!"),
        Err(err)=>println!("Error: {}", err)
    }
     match user2.withdraw(900.0){
        Ok(())=>println!("Withdrawal successful!"),
        Err(err)=>println!("Error: {}", err)
     }

    println!("Jumana's balance= {}", user1.balance());
    println!("Sarah's balance= {}", user2.balance());
}
