use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

use day7::{search_all_dirs, Data, Entry, DISK_SPACE, MIN_SPACE};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let lines: Vec<&str> = buffer.split_terminator("\n").collect();

    let root = Rc::new(RefCell::new(Entry::new("/", None)));

    let mut current_dir = Rc::clone(&root);

    // Build tree
    for line in lines {
        let components: Vec<&str> = line.split_terminator(" ").collect();

        if line.contains("$") {
            // Is a components
            if components[1] == "cd" {
                current_dir = {
                    // Argument
                    if components[2] == "/" {
                        Rc::clone(&root)
                    } else if components[2] == ".." {
                        Rc::clone(match &current_dir.borrow_mut().parent {
                            Some(parent) => &parent,
                            None => &current_dir,
                        })
                    } else {
                        match current_dir.borrow_mut().get_child(components[2]) {
                            Some(child) => child,
                            None => Rc::clone(&current_dir),
                        }
                    }
                };
            }
        } else {
            // Is a directory
            if components[0] == "dir" {
                let node = Rc::new(RefCell::new(Entry::new(
                    components[1],
                    Some(current_dir.clone()),
                )));
                current_dir.borrow_mut().add_dir(node);
            } else {
                let file = Data {
                    name: components[1],
                    size: components[0].parse().unwrap(),
                };
                current_dir.borrow_mut().add_file(file);
            }
        }
    }
    println!("{}", root.borrow_mut().print());
    let left_capacity = DISK_SPACE - root.borrow_mut().get_size();
    let delete_space: u64 = MIN_SPACE - left_capacity;
    println!("Remaining capacity:{}", left_capacity);
    println!("Need to delete:{}", delete_space);
    let all_dirs = search_all_dirs(root);

    let filtered_dirs: Vec<Rc<RefCell<Entry>>> = all_dirs
        .into_iter()
        .filter(|x| x.borrow_mut().get_size() > delete_space)
        .collect();

    let mut min_disk = DISK_SPACE;

    // let mut total: u64 = 0;
    for dir in filtered_dirs {
        let size = dir.borrow_mut().get_size();
        if size < min_disk {
            min_disk = size;
        }
    }
    println!("Smallest dir {}", min_disk);
}
