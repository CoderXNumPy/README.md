use std::io;

fn main() {
    println!("-------->Welcome to a game guessing game<-----------");
    
    println!("Guess between 1 to 10");
    
    let secret_number = 7;
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error");
        let guess: i32 = guess.trim().parse().unwrap();
        
        if guess == secret_number {
            println!("Congo You won ");
            break;
        } else {
            println!("You Loose Noob Loser ");
        }
    }
}
