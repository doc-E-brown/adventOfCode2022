use std::{cell::RefCell, rc::Rc};

pub const MAX_SIZE: u64 = 100000;
pub const MIN_SPACE: u64 = 30000000;
pub const DISK_SPACE: u64 = 70000000;

#[derive(Debug, PartialEq, Eq)]
pub enum DataType {
    File,
    Directory,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Data<'a> {
    pub name: &'a str,
    pub size: u64,
}

impl<'a> Data<'a> {
    pub fn new(name: &'a str, size: u64) -> Self {
        Data { name, size }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Entry<'a> {
    pub name: &'a str,
    pub contents: Vec<Data<'a>>,
    pub children: Vec<Rc<RefCell<Entry<'a>>>>,
    pub parent: Option<Rc<RefCell<Entry<'a>>>>,
}

impl<'a> Entry<'a> {
    pub fn new(name: &'a str, parent: Option<Rc<RefCell<Entry<'a>>>>) -> Self {
        Entry {
            name,
            contents: Vec::new(),
            children: Vec::new(),
            parent: parent,
        }
    }

    pub fn add_file(&mut self, file: Data<'a>) {
        self.contents.push(file);
    }

    pub fn add_dir(&mut self, node: Rc<RefCell<Entry<'a>>>) {
        self.children.push(node);
    }

    pub fn get_child(&self, name: &'a str) -> Option<Rc<RefCell<Entry<'a>>>> {
        for child in &self.children {
            if child.borrow_mut().name == name {
                return Some(Rc::clone(child));
            }
        }
        None
    }

    pub fn get_size(&self) -> u64 {
        let mut size: u64 = 0;
        for file in &self.contents {
            size += file.size;
        }

        for child in &self.children {
            size += child.borrow_mut().get_size();
        }
        size
    }

    pub fn print(&self) -> String {
        format!("{} {}", self.name, self.get_size())
    }
}

pub fn search_all_dirs<'a>(root: Rc<RefCell<Entry<'a>>>) -> Vec<Rc<RefCell<Entry<'a>>>> {
    let mut all_dirs: Vec<Rc<RefCell<Entry<'a>>>> = Vec::new();

    for child in &root.borrow_mut().children {
        all_dirs.push(Rc::clone(child));
        all_dirs.extend(search_all_dirs(Rc::clone(child)));
    }

    all_dirs
}
