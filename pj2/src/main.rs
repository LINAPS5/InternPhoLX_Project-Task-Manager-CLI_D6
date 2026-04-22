use std::fs;
use std::io;

const FILE_NAME: &str = "tasks.txt";

struct Task {
    title: String,
    done: bool,
}

fn main() {
    let mut tasks = load_task();
    loop {
        menu();

        let choice = input("Choose menu: ");
        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => show_tasks(&tasks),
            "3" => complete_task(&mut tasks),
            "4" => edit_task(&mut tasks),
            "5" => delete_task(&mut tasks),
            "6" => {
                save_tasks(&tasks);
                println!("Saved to {}. Exit", FILE_NAME);
                break;
                
            }
            _ => println!("Please try again"),
        }
    }
}

fn menu(){
    println!("=====================================");
    println!("1.Add Task");
    println!("2.Show Tasks");
    println!("3.Complete Task");
    println!("4.Edit Task");
    println!("5.Delete Task");
    println!("6.Exit");
}

fn input(prompt: &str) -> String{
    let mut text = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut text).expect("Failed to read input");
    text.trim().to_string()
}

fn add_task(tasks: &mut Vec<Task>){
    let title = input("Enter task title:");
    if title.is_empty() {
        println!("Task title cannot be empty");
        return;
    }

    tasks.push(Task {
        title,
        done: false,
    });
    println!("Task added");
}

fn show_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks");
        return;
    }

    println!("\n=== Task List ===");
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.done { "[O]" } else { "[ ]" };
        println!("{}. {} {}", i + 1, status, task.title);
    }
}

fn complete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks to complete");
        return;
    }

    show_tasks(tasks);

    let choice = input("Choose task number to complete:");
    let index: usize = match choice.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    if index == 0 || index > tasks.len() {
        println!("Out of range");
        return;
    }

    tasks[index - 1].done = true;
    println!("Task marked as completed");
}
fn edit_task(tasks: &mut Vec<Task>){
    if tasks.is_empty() {
        println!("No tasks to edit");
        return;
    }
    show_tasks(tasks);

    let choice = input("Choose task number to edit");
    let index: usize = match choice.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };
    if index == 0 || index > tasks.len() {
        println!("Out of range");
        return;
    }
    let new_title = input("Enter new title");
    if new_title.is_empty() {
        println!("Enter new title: ");
        return;
    }
    tasks[index - 1].title = new_title;
    println!("Task updated");
}
fn delete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No task to delete");
        return;
    }

    show_tasks(tasks);

    let choice = input("Choose task number to delete:");
    let index: usize = match choice.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    if index == 0 || index > tasks.len() {
        println!("Out of range");
        return;
    }

    tasks.remove(index - 1);
    println!("Task deleted");
}
fn load_task() -> Vec<Task> {
    let content = match fs::read_to_string(FILE_NAME) {
        Ok(text) => text,
        Err(_) => return Vec::new(),
    };
    let mut tasks: Vec<Task> = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.splitn(2, '|').collect();

        if parts.len() !=2 {
            continue;
        }

        let done = parts[0] == "1";
        let title = parts[1].to_string();

        tasks.push(Task { title, done });
    }
    tasks
}

fn save_tasks(tasks: &Vec<Task>) {
    let mut content = String::new();

    for task in tasks {
        let done = if task.done { "1" } else { "0" };
        content.push_str(&format!("{}|{}\n",done,task.title));

    }
    if let Err(error) = fs::write(FILE_NAME, content) {
        println!("Failed to save file: {}",error);
    }
}
