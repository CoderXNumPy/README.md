use std::io;

fn main() {
    println!("Enter first Number" );
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).unwrap();
    let num1: i32 = num1.trim().parse().unwrap();
    
    println!("Enter second number number");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).unwrap();
    let num2: i32 = num2.trim().parse().unwrap();
    
    println!("Enter Op + - * /");
    let mut op = String::new();
    io::stdin().read_line(&mut op).unwrap();
    let op = op.trim();
    match op {
        "+" => println!("{} + {} = {}",num1, num2, num1 + num2),
        "-" => println!("{} - {} = {}",num1, num2, num1 - num2),
        "*" => println!("{} * {} = {}",num1, num2, num1 * num2),
        "/" => println!("{} / {} = {}",num1, num2, num1 / num2),
        _ => println!("Invalid "),

  }
}
