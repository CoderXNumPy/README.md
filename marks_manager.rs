use std::io;

fn main() {
    let mut marks: Vec<i32> = Vec::new();
    
    for i in 0..5 {
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
    
    let mut sum = 0;
    for m in &marks {
        sum = sum + *m;
    }
    println!("Sum {}", sum);
    
    let average = sum / marks.len() as i32;
    println!("Average: {}", average);
    
    let mut max = marks[0];
    for m in &marks {
        if *m > max {
            max = *m;
        }
    }
    println!("Topper: {}", max);
    
    let mut min = marks[0];
    
    for m in &marks {
       if *m < min {
           min = *m;
       }
    }
    println!("Lowest: {}", min);
}  
