use std::io;

fn main() {
    let mut marks: Vec<i32> = Vec::new();
    
    for _ in 0..5 {
        println!("Enter marks");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Galat number, phir try karo!");
                continue;
            }
        };
        marks.push(input);
    }
    println!("Marks {:?}", marks);
}
