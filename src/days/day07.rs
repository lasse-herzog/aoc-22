use crate::io::read_lines;
use std::cell::RefCell;
use std::mem::swap;
use std::rc::Rc;

struct FileSystemNode {
    pub children: Vec<Rc<RefCell<FileSystemNode>>>,
    pub parent: Option<Rc<RefCell<FileSystemNode>>>,
}

struct DirectoryNode {
    pub name: Option<String>,
    pub node: FileSystemNode,
}

struct FileNode {
    pub name: Option<String>,
    pub node: FileSystemNode,
    pub size: u32,
}

pub fn day07() {
    if let Ok(input) = read_lines("../../../day06.txt") {
        let mut current_directory = "";
        let mut last_directory = "";

        let console_history = input.map(|line| {
            let console_line = line.unwrap();

            if console_line.starts_with("$") {
                let command: Vec<&str> = console_line.split(" ").collect();
                match command[1] {
                    "cd" => {
                        if command[2] == ".." {
                            swap(&mut current_directory, &mut last_directory)
                        } else {
                            last_directory = current_directory;
                            current_directory = command[2]
                        }
                    }
                    "ls" => {}
                    _ => {}
                }
            }
        });
    }
}
