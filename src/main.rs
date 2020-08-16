use std::env;

struct TodoItem {
    name: String,
    completed: char,
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        TodoItem {
            name: name,
            completed: ' ',
        }
    }
}

struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { list: Vec::new() }
    }
    fn add(&mut self, name: String) {
        self.list.push(TodoItem::new(name))
    }

    fn toggle_completed(&mut self, index: usize) {
        if self.list[index].completed == ' ' {
            self.list[index].completed = 'x';
        } else {
            self.list[index].completed = ' ';
        }
    }

    fn remove_task(&mut self, index: usize) {
        self.list.remove(index);
    }

    fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{} [{}] - {}", index, item.completed, item.name);
        }
    }
}

enum Command {
    Get,
    Add(String),
    Done(usize),
    Remove(usize),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::new();

    let command = match args[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(args[2].clone()),
        "done" => Command::Done(
            args[2]
                .parse()
                .expect("Error converting to integer for done"),
        ),
        "remove" => Command::Remove(
            args[2]
                .parse()
                .expect("Error converting to integer for remove"),
        ),
        _ => panic!("You must provide a command!"),
    };
    todo_list.add("Say hi".to_string());
    todo_list.add("Do something with Rust".to_string());

    match command {
        Command::Get => todo_list.print(),
        Command::Add(name) => {
            todo_list.add(name);
            todo_list.print();
        }
        Command::Done(index) => {
            todo_list.toggle_completed(index);
            todo_list.print();
        }
        Command::Remove(index) => {
            todo_list.remove_task(index);
            todo_list.print();
        }
    }
}
