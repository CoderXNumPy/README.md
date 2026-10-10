use std::io;

struct Task {
    title: String,
    done: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    
    loop {
        println!("===== To-Do List =====");
        println!("1. Add a new task");
        println!("2. Show all tasks");
        println!("3. Mark task as done");
        println!("4. Exit");
        println!("What do you want to do? ");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        
        match choice {
            "1" => {
                println!("Enter task title: ");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim().to_string();
                
                let new_task = Task {
                    title: title,
                    done: false,
                };
                tasks.push(new_task);
                println!("Task Added");
            }
            "2" => {
                for t in &tasks {
                    if t.done {
                        println!("✅ {}", t.title);
                    } else {
                        println!("❌ {}", t.title);
                    }
                }
            }
            "3" => {
                for (i, t) in tasks.iter().enumerate() {
                    println!("{}. {}", i + 1, t.title);
                }
                
                println!("Enter task number: ");
                let mut num = String::new();
                io::stdin().read_line(&mut num).unwrap();
                let num: usize = num.trim().parse().unwrap();
                
                if num > tasks.len() || num == 0 {
                    println!("Invalid task number!");
                } else {
                    tasks[num - 1].done = true;
                    println!("Task marked as done!");
                }
            }
            "4" => {
                println!("Bye!");
                break;
            }
            _ => {
                println!("Invalid option!");
            }
        }
    }
}
