use std::io;

struct Task {
	title: String,
	done: bool,
}

fn main() {
	let mut tasks: Vec<Task> = Vec::new();
  loop {
      menu();

      let choice = input("Choose menu: ");
      match choice.trim(){
          "1" => add_task(&mut tasks),
          "2" => show_tasks(&mut tasks),
          "3" => complete_task(&mut tasks),
          "4" => delete_task(&mut tasks),
          "5" => {
              println!("Exit");
              break;
          }
          _ => println!("Please try again"),
      }
  }
    
}
fn menu(){
	println!("============================");
	println!("Task Manager CLI");
	println!("1.Add Task");
	println!("2.Show Tasks");
	println!("3.Complete Task");
	println!("4.Delete Task");
	println!("5.Exit");	
}
fn input(prompt: &str) -> String{
	let mut text = String::new();
	println!("{}",prompt);
	io::stdin().read_line(&mut text).expect("Failed read input");
	text.trim().to_string()
}

fn add_task(tasks: &mut Vec<Task>){
	let title = input("Enter title:");
	if title.is_empty() {
		println!("Task title cannot be empty");
		return;	
	}
	let task = Task {
		title,
		done: false,
	};
	tasks.push(task);
	println!("Task added");
}

fn show_tasks(tasks: &Vec<Task>){
    if tasks.is_empty() {
        println!("No tasks");
        return;
    }
    println!("Task List");
    for (i,task) in tasks.iter().enumerate(){
        let status = if task.done { "[O]" } else { "[  ] "};
        println!("{}. {} {}", i+1, status,task.title);
    }
}
fn complete_task(tasks: &mut Vec<Task>){
    if tasks.is_empty() {
        println!("No tasks Complete");
        return;
    }
    show_tasks(tasks);

    let choice = input("choice task to complete:");
    let index: usize = match choice.parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };
    if index == 0 || index > tasks.len(){
        println!("out of range");
        return;
    }
    tasks[index - 1].done = true;
    println!("mark Completed");
    
}
fn delete_task(tasks: &mut Vec<Task>){
    if tasks.is_empty() {
        println!("no task to delete");
        return;
    }
    show_tasks(tasks);
    let choice = input("Choose to delete:");
    let index: usize = match choice.parse(){
        Ok(num) => num,
        Err(_) => {
            println!("valid number");
            return;
        }
    };
    if index == 0 || index > tasks.len(){
        println!("out of range");
        return;
    }
    tasks.remove(index - 1);
    println!("Task deleted");
}
