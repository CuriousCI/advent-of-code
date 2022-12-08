// use std::{collections::HashMap, ops::Deref};

#[derive(Debug)]
struct Folder {
    files_size: u32,
    folders: Vec<Folder>,
}

impl Folder {
    fn new() -> Self {
        Self {
            files_size: 0,
            folders: vec![],
        }
    }

    fn size(&self) -> u32 {
        return self.files_size + self.folders.iter().map(|folder| folder.size()).sum::<u32>();
    }
}

fn part_one<'a>() -> u32 {
    let mut path: Vec<&str> = vec![];
    // let mut folders: HashMap<String, Folder> = HashMap::new();

    let commands: Vec<(&str, &str)> = include_str!("input")
        .trim_start_matches("$ ")
        // .replace("cd ", "cd\n")
        .split("\n$ ")
        .map(|command| command.split_once("\n").unwrap())
        .collect();

    println!("{:?}", commands);

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
