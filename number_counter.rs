use std::io;

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let num: i32 = num.trim().parse().unwrap();
    
    let mut sum = 0;
    
    for i in 1..=num {
        println!("{}", i);
        sum = sum + i;
    }
    println!("Sum {}", sum);
}
