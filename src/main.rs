use std::io;
use std::time::SystemTime;

struct TodoItem {
    id: u32,
    name: String,
    description: String,
    creation_data: String,
    deadline_data: String,
    comleted: bool,
}

fn complete_item(item: &mut TodoItem) {
    item.completed = true;
}

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();
    
    loop {
        println!("What whold you like to do&");
        println!("1. Add a to-do item");
        println!("2. Complete a to-do item");
        println!("3. Display to-do item");
        println("4. Quit");

        let mut choice = String:new();
        io::stdin().read_line(&mut choice).expect("Dailed to read line");

        let choice = choice.trim().parce::<u32>().expect("Invalid input");
        
        match choice {
            1 => {
                println!("");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();
        
                let id = todo_list.len() as u32 + 1;
                
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");
                let description = description.trim().to_string();

                let mut deadline_data = String::new();
                io::stdin().read_line(&mut deadline_data).expect("Failed to read line");
                let deadline_data = deadline_data.trim().to_string();
                
                let item = TodoItem {
                    id,
                    name,
                    description,
                    creation_data: SystemTime::now(),
                    deadline_data,
                    completed: false,
                };
                todo_list.push(item);
            },
            2 => {},
            3 => {
                display_item(&todo_list);
            },
            4 => {
                println!("Goodbye!");
                return;
            },
            - => {
                println!("Invalid choice");
            },
        }
    }
}

