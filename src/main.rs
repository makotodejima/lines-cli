use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, BufRead};

struct Item {
    name: String,
}

impl Item {
    fn new(name: String) -> Item {
        Item { name: name }
    }
}

struct ItemList {
    list: Vec<Item>,
}

impl ItemList {
    fn new() -> ItemList {
        ItemList { list: Vec::new() }
    }

    fn load(&mut self, path: String) {
        let file = File::open(path).expect("Can not find file");
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            if let Ok(ln) = line {
                self.list.push(Item::new(ln));
            }
        }
    }

    fn add(&mut self, name: String) {
        let mut file_by_line = OpenOptions::new()
            .append(true)
            .open("lines.txt")
            .expect("Can not find file");
        if let Err(e) = writeln!(file_by_line, "{}", name) {
            eprintln!("could not write line: {}", e);
        }
        self.list.push(Item::new(name))
    }

    fn remove_item(&mut self, index: usize) {
        self.list.remove(index);
    }

    fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{} - {}", index, item.name);
        }
    }
}

enum Command {
    Get,
    Add(String),
    Remove(usize),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut item_list = ItemList::new();

    item_list.load("lines.txt".to_string());

    if args.len() == 1 {
        panic!("No args!!");
    }

    let command = match args[1].as_str() {
        "add" => Command::Add(args[2].clone()),
        "remove" => Command::Remove(
            args[2]
                .parse()
                .expect("Error converting to integer for remove"),
        ),
        "get" | _ => Command::Get,
    };

    match command {
        Command::Get => item_list.print(),
        Command::Add(name) => {
            item_list.add(name);
            item_list.print();
        }
        Command::Remove(index) => {
            item_list.remove_item(index);
            item_list.print();
        }
    }
}
