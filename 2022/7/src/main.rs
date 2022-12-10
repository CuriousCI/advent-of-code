use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Folder {
    files: RefCell<u32>,
    folders: RefCell<HashMap<String, Rc<Folder>>>,
    parent: Option<Rc<Folder>>,
}

impl Folder {
    fn new(parent: Option<Rc<Folder>>) -> Self {
        Self {
            files: RefCell::new(0),
            folders: RefCell::new(HashMap::new()),
            parent,
        }
    }

    fn insert(&self, name: String, folder: Rc<Folder>) {
        self.folders.borrow_mut().insert(name, folder);
    }

    fn size(&self) -> u32 {
        *self.files.borrow()
            + self
                .folders
                .borrow()
                .values()
                .map(|folder| folder.size())
                .sum::<u32>()
    }
}

fn part_one() -> u32 {
    let file = include_str!("input")
        .trim_start_matches("$ ")
        .replace("ls\n", "ls ");

    let commands: Vec<(&str, &str)> = file
        .split("\n$ ")
        .map(|command| command.split_once(" ").unwrap())
        .collect();

    let mut folders: Vec<Rc<Folder>> = vec![];

    let root = Rc::new(Folder::new(None));
    let mut folder = Rc::clone(&root);

    for (left, right) in commands {
        match left {
            "cd" => {
                folder = match right {
                    "/" => Rc::clone(&root),
                    ".." => Rc::clone(&folder.parent.as_ref().unwrap()),
                    _ => Rc::clone(&folder.folders.borrow().get(right).unwrap()),
                }
            }
            "ls" => {
                for (info, name) in right.lines().map(|line| line.split_once(" ").unwrap()) {
                    match info {
                        "dir" => {
                            let f = Rc::new(Folder::new(Some(Rc::clone(&folder))));
                            folders.push(Rc::clone(&f));
                            folder.insert(
                                name.to_string(),
                                Rc::clone(&f), // Rc::new(Folder::new(Some(Rc::clone(&folder)))),
                            );
                        }
                        _ => {
                            let size: u32 = info.parse().unwrap();
                            *folder.files.borrow_mut() += size;
                        }
                    }
                }
            }
            _ => unreachable!(),
        };
    }

    folders
        .iter()
        .map(|folder| folder.size())
        .filter(|size| *size <= 100000)
        .sum()
}

fn part_two() -> u32 {
    let file = include_str!("input")
        .trim_start_matches("$ ")
        .replace("ls\n", "ls ");

    let commands: Vec<(&str, &str)> = file
        .split("\n$ ")
        .map(|command| command.split_once(" ").unwrap())
        .collect();

    let mut folders: Vec<Rc<Folder>> = vec![];

    let root = Rc::new(Folder::new(None));
    let mut folder = Rc::clone(&root);

    for (left, right) in commands {
        match left {
            "cd" => {
                folder = match right {
                    "/" => Rc::clone(&root),
                    ".." => Rc::clone(&folder.parent.as_ref().unwrap()),
                    _ => Rc::clone(&folder.folders.borrow().get(right).unwrap()),
                }
            }
            "ls" => {
                for (info, name) in right.lines().map(|line| line.split_once(" ").unwrap()) {
                    match info {
                        "dir" => {
                            let f = Rc::new(Folder::new(Some(Rc::clone(&folder))));
                            folders.push(Rc::clone(&f));
                            folder.insert(
                                name.to_string(),
                                Rc::clone(&f), // Rc::new(Folder::new(Some(Rc::clone(&folder)))),
                            );
                        }
                        _ => {
                            let size: u32 = info.parse().unwrap();
                            *folder.files.borrow_mut() += size;
                        }
                    }
                }
            }
            _ => unreachable!(),
        };
    }

    let total_space: i32 = 70000000;
    let required_space: i32 = 30000000;

    let occupied_space: i32 = root.size() as i32;

    let mut sizes: Vec<u32> = folders.iter().map(|folder| folder.size()).collect();
    sizes.sort();

    for size in sizes {
        if (total_space - occupied_space) + size as i32 >= required_space {
            return size;
        }
    }

    unreachable!()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
