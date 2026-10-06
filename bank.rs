struct Account {
    acc_no: u32,
    holder: String,
    balance: f64,
    history: Vec<String>,
}

impl Account {
    fn deposit(&mut self, amount: f64) {
        self.balance = self.balance + amount;
        println!("Deposited {}", amount);
        self.history.push(format!("Deposited {}", amount));
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            println!("Balance kam hai brotha");
            return;
        }
        self.balance = self.balance - amount;
        println!("Withdrawal {}", amount);
        self.history.push(format!("Withdrawn {}", amount));
    }

    fn show_balance(&self) {
        println!("Account no {} balance {}", self.acc_no, self.balance);
    }

    fn show_history(&self) {
        println!("---------Transaction History--------- ");
        for t in &self.history {
            println!("{}", t);
        }
    }
}

fn main() {
    let mut acc1 = Account {
        acc_no: 101,
        holder: String::from("XyLon"),
        balance: 1000.0,
        history: Vec::new(),
    };

    acc1.deposit(500.0);
    println!("Balance: {}", acc1.balance);

    acc1.withdraw(200.0);
    println!("Balance: {}", acc1.balance);

    acc1.show_history();
} // thanks for seeing my code // rust 
