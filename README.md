# To-Do List Application in Rust

## Overview

This project implements a simple command-line To-Do list application in Rust. It allows users to:

- Add, remove, and edit tasks.
- Mark tasks as completed.
- View all tasks.
- Save tasks to a file and load them back from a file using JSON serialization.
  The application uses basic Rust concepts such as structs, enums, and collections, as well as file I/O and serialization.

## Features

1. **Task Management**:

   - Add, edit, and remove tasks.
   - Mark tasks as completed.

2. Task Viewing:

   - View all tasks with their details: ID, description, and completion status.

3. Persistent Storage:

   - Save tasks to a file in JSON format.
   - Load tasks from a file.

4. Menu-Driven Interface:

   - Users interact with the program through a simple menu with clear options.

## Example User Interaction

```plaintext
Welcome to the todo list app
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

What would you like to do : 1
Enter task description: Buy groceries
Task added

What would you like to do : 3
[Task { id: 1, description: "Buy groceries", completed: false }]
What would you like to do : 5
Enter task id to mark as completed: 1
Task marked as completed

What would you like to do : 6
Enter file path: tasks.json
Tasks saved to file

What would you like to do : 7
Enter file path: tasks.json
Tasks loaded from file

What would you like to do : 8
Goodbye!
```

## Code Explanation

1. **Structs**:

   - `Task`: Represents a task with an ID, description, and completion status.
     - Methods:
       - `mark_as_completed`: Marks a task as completed.
       - `edit_description`: Edits the description of a task.

2. **Enums**:

   - `MenuOption`: Represents different options available in the menu. It includes options for adding, removing, and viewing tasks, as well as saving and loading tasks.

3. **Functions**:

   - `add_task`: Adds a task to the list with a unique ID.
   - `remove_task`: Removes a task from the list by ID.
   - `view_tasks`: Displays all tasks.
   - `save_to_file`: Serializes the task list to a file in JSON format.
   - `load_from_file`: Deserializes the task list from a JSON file.

4. **Input Handling**:

   - `get_input`: Prompts the user for input and reads the input from the command line.
   - `number_check`: Validates the input to ensure it is a valid number.

## How to Use

### Prerequisites

- Ensure you have Rust and Cargo installed. [Install Rust here](https://www.rust-lang.org/tools/install).

### Steps to Run

1. Clone the repository:

```sh
git clone https://github.com/yourusername/todo-list-app.git
```

2. Navigate to the project directory:

```sh
cd todo-list-app
```

3. Run the program:

```sh
cargo run
```

## Future Enhancements

- **Task Prioritization**:

  - Add priority levels to tasks (e.g., Low, Medium, High).

- **Due Dates**:

  - Allow tasks to have due dates, and sort tasks by their due date.

- **Task Categories**:

  - Allow tasks to be categorized (e.g., Work, Personal, Shopping).

- **Search Tasks**:

  - Add a search functionality to find tasks based on their description.

- **User Authentication**:

  - Implement user authentication to manage individual task lists.
