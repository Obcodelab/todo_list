mod input;
use input::{get_input, number_check};
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, write};
use std::io::Result;
fn main() {
    #[derive(Serialize, Deserialize, Debug)]
    struct Task {
        id: u32,
        description: String,
        completed: bool,
    }

    impl Task {
        fn mark_as_completed(&mut self) {
            self.completed = true;
        }

        fn edit_description(&mut self, new_description: String) {
            self.description = new_description;
        }
    }

    enum MenuOption {
        AddTask,
        RemoveTask,
        ViewTasks,
        EditTask,
        MarkCompleted,
        Save,
        Load,
        Exit,
    }

    impl MenuOption {
        fn from_input(input: &str) -> Option<MenuOption> {
            match input {
                "1" => Some(MenuOption::AddTask),
                "2" => Some(MenuOption::RemoveTask),
                "3" => Some(MenuOption::ViewTasks),
                "4" => Some(MenuOption::EditTask),
                "5" => Some(MenuOption::MarkCompleted),
                "6" => Some(MenuOption::Save),
                "7" => Some(MenuOption::Load),
                "8" => Some(MenuOption::Exit),
                _ => None,
            }
        }
    }

    fn add_task(tasks: &mut Vec<Task>, description: String) {
        let id = tasks.len() as u32 + 1;
        tasks.push(Task {
            id,
            description,
            completed: false,
        });
    }

    fn remove_task(tasks: &mut Vec<Task>, id: u32) {
        tasks.retain(|task| task.id != id);
    }

    fn view_tasks(tasks: &Vec<Task>) {
        for task in tasks {
            println!("{:?}", task);
        }
    }

    fn save_to_file(tasks: &Vec<Task>, file_path: &str) -> Result<()> {
        let serialized = serde_json::to_string(&tasks)?;
        write(file_path, serialized)?;
        Ok(())
    }
    fn load_from_file(file_path: &str) -> Vec<Task> {
        match read_to_string(file_path) {
            Ok(data) => {
                println!("Loaded file content: {}", data);
                match serde_json::from_str(&data) {
                    Ok(tasks) => tasks,
                    Err(e) => {
                        println!("Error deserializing tasks from file: {}", e);
                        vec![]
                    }
                }
            }
            Err(e) => {
                println!("Error reading file: {}", e);
                vec![]
            }
        }
    }

    let mut tasks = Vec::new();
    let mut active = true;
    println!("Welcome to the todo list app");
    while active {
        println!(
            "
         _______________________
        | 1. Add Task           | 
        | 2. Remove Task        |   
        | 3. View Tasks         |
        | 4. Edit Task          |
        | 5. Mark Completed     |
        | 6. Save               |
        | 7. Load               |
        | 8. Exit               |
         _______________________
        "
        );
        let choice = get_input("What would you like to do : ");
        if let Some(option) = MenuOption::from_input(&choice) {
            match option {
                MenuOption::AddTask => {
                    let description = get_input("Enter task description: ");
                    add_task(&mut tasks, description);
                    println!("Task added");
                }
                MenuOption::RemoveTask => {
                    let id_str = get_input("Enter task id: ");
                    let id = number_check(&id_str);
                    if id > 0 {
                        remove_task(&mut tasks, id);
                        println!("Task removed");
                    } else {
                        println!("Task with ID {} not found", id);
                    }
                }
                MenuOption::ViewTasks => {
                    if tasks.len() > 0 {
                        view_tasks(&tasks);
                    } else {
                        println!("No task added yet");
                    }
                }
                MenuOption::EditTask => {
                    let id_str = get_input("Enter task id to edit: ");
                    let id = number_check(&id_str);
                    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                        let new_description = get_input("Enter new task description: ");
                        task.edit_description(new_description);
                        println!("Task description updated");
                    } else {
                        println!("Task with ID {} not found", id);
                    }
                }
                MenuOption::MarkCompleted => {
                    let id_str = get_input("Enter task id to mark as completed: ");
                    let id = number_check(&id_str);
                    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                        task.mark_as_completed();
                        println!("Task marked as completed");
                    } else {
                        println!("Task with ID {} not found", id);
                    }
                }
                MenuOption::Save => {
                    let file_path = get_input("Enter file path: ");
                    if let Err(e) = save_to_file(&tasks, &file_path) {
                        println!("Failed to save tasks: {}", e);
                    } else {
                        println!("Tasks saved to file");
                    }
                }
                MenuOption::Load => {
                    let file_path = get_input("Enter file path: ");
                    tasks = load_from_file(&file_path);
                    println!("Tasks loaded from file");
                }
                MenuOption::Exit => active = false,
            }
        } else {
            println!("Invalid input");
        }
    }
    println!("Goodbye!");
}
