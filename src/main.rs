use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::PathBuf;

struct Item {
    text: String,
}

impl Item {
    fn new(text: String) -> Item {
        Item { text: text }
    }
}

struct Set {
    items: Vec<Item>,
}

impl Set {
    fn new() -> Set {
        Set { items: Vec::new() }
    }

    fn load(&mut self, path: PathBuf) {
        let file = File::open(path).expect("Can not find file");
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            if let Ok(ln) = line {
                self.items.push(Item::new(ln));
            }
        }
    }

    fn add(&mut self, text: String) {
        let mut file = OpenOptions::new()
            .append(true)
            .open("lines.txt")
            .expect("Can not find file");
        if let Err(e) = writeln!(file, "{}", text) {
            eprintln!("could not write line: {}", e);
        }
        self.items.push(Item::new(text))
    }

    fn remove_item(&mut self, index: usize) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("lines.txt")
            .expect("Can not find file");

        self.items.remove(index);

        for item in self.items.iter() {
            writeln!(file, "{}", item.text).expect("Can not write lines");
        }
    }

    fn print(&self) {
        for (index, item) in self.items.iter().enumerate() {
            println!("{} - {}", index, item.text);
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
    let mut item_list = Set::new();
    let path = PathBuf::from("lines.txt");

    item_list.load(path);

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
        Command::Add(text) => {
            item_list.add(text);
            item_list.print();
        }
        Command::Remove(index) => {
            item_list.remove_item(index);
            item_list.print();
        }
    }
}
