use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::env::{current_dir, current_exe};
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    path: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    parent: Option<Weak<RefCell<Folder>>>,
    subfolders: Vec<Rc<RefCell<Folder>>>,
    files: Vec<ElfFile>, // <name, size>
}

#[derive(Debug)]
struct ElfFile {
    name: String,
    size: u64,
}

impl Folder {
    fn new_root(name: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Folder {
            name: "".to_string(),
            parent: None,
            subfolders: vec![],
            files: vec![],
        }))
    }

    fn new_with_parent(name: &str, parent: &Rc<RefCell<Folder>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Folder {
            name: name.to_string(),
            parent: Some(Rc::downgrade(parent)),
            subfolders: vec![],
            files: vec![],
        }))
    }

    fn add_file(&mut self, name: &str, size: u64) {
        self.files.push(ElfFile {
            name: name.to_string(),
            size,
        })
    }

    fn add_subfolder(&mut self, subfolder: Rc<RefCell<Folder>>) {
        self.subfolders.push(subfolder);
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Folder>>> {
        self.parent.as_ref().and_then(Weak::upgrade)
    }

    fn find_subfolder(&self, name: &str) -> Option<Rc<RefCell<Folder>>> {
        for subfolder in &self.subfolders {
            if subfolder.borrow().name == name {
                return Some(subfolder.clone());
            }
        }
        None
    }
}

fn main() {
    let args = Args::parse();
    let path = args.path;

    let root_folder = Folder::new_root("/");
    let mut current_folder = root_folder.clone();

    let mut lines = read_lines(path).unwrap();
    for line in lines.by_ref().flatten().skip(1) {
        println!("Processing line {}; Current dir is {}", line, current_folder.borrow().name);
        if line.starts_with("$") {
            current_folder = execute_command(&line, current_folder.clone());
        }

        if line.starts_with("dir") {
            store_dir(current_folder.clone(), line.clone());
        }
        if line.chars().nth(0).unwrap().is_numeric() {
            // if first char in line is a number - this is a file listing
            store_file(current_folder.clone(), line.clone());
        }
    }
}

fn store_file(folder: Rc<RefCell<Folder>>, line: String) {
    let command_parase_r = Regex::new(r"(\d+) (\w+.?\w+)").unwrap();
    let c = command_parase_r.captures(line.as_str()).unwrap();
    let size: u64 = c[1].parse().expect("Failed to parse a file size");
    let name = c[2].to_string();
    println!("Adding file {}; Size {} to folder {}", line, size, folder.borrow().name);
    folder.borrow_mut().add_file(&name.to_string(), size);
}

fn store_dir(folder: Rc<RefCell<Folder>>, line: String) {
    let command_parase_r = Regex::new(r"dir (\w+)").unwrap();
    let c = command_parase_r.captures(line.as_str()).unwrap();
    let dir_name = &c[1];
    println!("Storing dir {}", dir_name);
    let subfolder1 = Folder::new_with_parent(dir_name, &folder);
    folder.borrow_mut().add_subfolder(subfolder1);
}

fn execute_command(line: &str, current_folder: Rc<RefCell<Folder>>) -> Rc<RefCell<Folder>> {
    let command_parase_r = Regex::new(r"\$ (\w+)\s*(\w+|\/|..)?").unwrap();
    let c = command_parase_r.captures(line).unwrap();
    let cmd = &c[1];
    let params = c.get(2).map_or("", |m| m.as_str());
    match cmd {
        "cd" => {
            println!("Changing dir for {}", params.to_string());
            if params.to_string().eq("..") {
                // go up
                return current_folder.borrow().get_parent().unwrap()
            }
            // find subfolder with name passed in poram
            // it should always find something
            current_folder.borrow().find_subfolder(params).unwrap()
        },
        _ => {
            println!("Unknown command: {}", cmd);
            current_folder
        }
    }
    //$ cd cwdpn
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

