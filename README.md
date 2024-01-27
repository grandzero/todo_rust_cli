# **Todo CLI Tool**

Welcome to the Todo CLI Tool, a command-line utility built in Rust for managing tasks and projects efficiently. This tool allows users to create and manage tasks within different projects, making task management seamless and straightforward.

## **Features**

- **Create Projects**: Organize your tasks into different projects.
- **Set Default Project**: Choose a default project for task management.
- **Create Tasks**: Add new tasks with titles, descriptions, and order.
- **Update Tasks**: Modify existing tasks easily.
- **View Task List**: Get a list of all tasks in the current project.
- **View Specific Task**: Retrieve details of a specific task by name.
- **Complete Tasks**: Mark tasks as complete.
- **Remove Tasks**: Delete tasks from the project.

## **Installation**

*Instructions on how to install the Todo CLI tool.*

For local use run : 
```bash
cargo run -- [COMMAND] --[SUBCOMMANDS]
```

## **Usage**

Below are the key commands you can use in the Todo CLI Tool:

### **Set Default Project**

To set a default project:

```bash
todo_cli --set-project --name "my-project"
```

### **Create a New Task**

To create a new task with a title, description, and order:

```bash
todo_cli --create-task --title "task title" --description "task description" --order 2
```

### **Update an Existing Task**

To update the title of an existing task:

```bash
todo_cli --update-task --id 1 --title "updated title"
```

### **View Task List**

To get a list of all tasks in the current project:

```bash
todo_cli --get-tasklist
```

### **View a Specific Task**

To get details of a specific task by its name:

```bash
todo_cli --get-task --name "task name"
```

### **Complete a Task**

To mark a task as complete:

```bash
todo_cli --complete --id 1
```

### **Remove a Task**

To remove a task from the project:

```bash
todo_cli --remove --id 1
```
