struct Account {
    acc_no: u32,
    holder: String,
    balance: f64,
}

impl Account {
    fn deposit(&mut self, amount: f64) {
        self.balance = self.balance + amount;
        println!("Deposited {}", amount);
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            println!("Balance kam hai brotha");
            return;
        }
        self.balance = self.balance - amount;
        println!("Withdrawal {}", amount);
    }
}

fn main() {
    let mut acc1 = Account {
        acc_no: 101,
        holder: String::from("XyLon"),
        balance: 1000.0,
    };

    acc1.deposit(500.0);
    println!("Balance: {}", acc1.balance);

    acc1.withdraw(200.0);
    println!("Balance: {}", acc1.balance);
}
