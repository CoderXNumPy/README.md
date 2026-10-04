use std::io; // Lolllllllllllllll

fn main() {
    let mut num = String::new();
    println!("Enter Number");
 
    io::stdin().read_line(&mut num).expect("Erro");
    let num: i32 = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Number Daalo");
            return;
            
          }
        
        };
        for i in 1..=10 {
            println!("{} x {} = {}",num , i, num * i);
            
    }

}
