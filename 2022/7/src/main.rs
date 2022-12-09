// use std::{collections::HashMap, ops::Deref};

// #[derive(Debug)]
// struct Folder {
//     files_size: u32,
//     folders: Vec<Folder>,
// }
//
//

use std::collections::HashMap;

#[derive(Debug)]
struct Folder<'a> {
    files: u32,
    folders: HashMap<String, Folder<'a>>,
    parent: Option<&'a Folder<'a>>,
}

impl<'a> Folder<'a> {
    fn get(&mut self, name: String, folder: Folder<'a>) -> &Folder<'a> {
        self.folders.entry(name).or_insert(folder)
    }
}

fn part_one<'a>() -> u32 {
    let file = include_str!("input")
        .trim_start_matches("$ ")
        .replace("ls\n", "ls ");

    let commands: Vec<(&str, &str)> = file
        .split("\n$ ")
        .map(|command| command.split_once(" ").unwrap())
        .collect();

    let root = Folder {
        files: 0,
        folders: HashMap::new(),
        parent: None,
    };

    let mut current_folder = &root;

    for (command, parameter) in commands {
        // get a reference to current directory! And insert it

        match command {
            "cd" => {
                current_folder = match parameter {
                    "/" => &root,
                    ".." => current_folder.parent.unwrap(),
                    _ => current_folder.get(
                        parameter.to_string(),
                        Folder {
                            files: 0,
                            folders: HashMap::new(),
                            parent: Some(&current_folder),
                        },
                    ),
                }
            }
            "ls" => {
                for (file_type, info) in parameter.lines().map(|line| line.split_once(" ").unwrap())
                {
                    match file_type {
                        "dir" => {
                            // add full path to current directory
                        }
                        _ => {
                            let size: u32 = info.parse().unwrap();
                            // add size to current directory
                        }
                    }
                }
            }
            _ => unreachable!(),
        };
    }
    // {
    //     match &command[..2] {
    //         "cd" => {
    //             let directory = &command[3..];
    //             match directory {
    //                 "/" => path.clear(),
    //                 ".." => {
    //                     path.pop();
    //                 }
    //                 _ => path.push(directory),
    //             }
    //         }
    //         "ls" => {
    //             for item in command[3..].lines() {
    //                 let mut folder = Folder::new();
    //
    //                 match &item[..3] {
    //                     "dir" => {
    //                         let directory = &item[4..];
    //                     }
    //                     _ => {
    //                         let (size, _) = item.split_once(" ").unwrap();
    //                         let size: u32 = size.parse().unwrap();
    //                         folder.files_size += size;
    //                     }
    //                 }
    //             }
    //         }
    //         _ => unreachable!(),
    //     };
    // }

    // for folder in folders {
    //     println!("{:?}", folder);
    // }
    // println!("{:?}", folders);

    10
}

fn main() {
    println!("{}", part_one());
}
