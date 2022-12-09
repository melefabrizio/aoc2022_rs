use std::fmt::{Debug, Formatter};

use crate::days::day7::Lines::Dir;

pub(crate) fn run() {
    part1();
}

struct File {
    name: String,
    size: i32,
    directories: Vec<String>,
}

struct Directory {
    name: String,
    files: Vec<File>,
    parent: Option<String>,
}

enum Lines {
    Cd(Directory),
    Ls,
    File(File),
    Dir(Directory),
}

impl Debug for Directory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Directory")
            .field("name", &self.name)
            .field("files", &self.files)
            .field("parent", &self.parent)
            .finish()
    }
}

impl Clone for Directory {
    fn clone(&self) -> Self {
        Directory {
            name: self.name.clone(),
            files: self.files.clone(),
            parent: self.parent.clone(),
        }
    }
}

impl Clone for File {
    fn clone(&self) -> Self {
        File {
            name: self.name.clone(),
            size: self.size,
            directories: self.directories.clone(),
        }
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("File")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("directories", &self.directories)
            .finish()
    }
}


fn parse_line(line: &str) -> Lines {
    let words = line.split_ascii_whitespace().collect::<Vec<&str>>();
    return if words[1] == "cd" {
        Lines::Cd(Directory { name: words[2].to_string(), files: vec![], parent: None })
    } else if words[1] == "ls" {
        Lines::Ls
    } else if words[0] == "dir" {
        Lines::Dir(Directory { name: words[1].to_string(), files: vec![], parent: None })
    } else {
        let name = words[1].to_string();
        let size = words[0].parse::<i32>().unwrap();
        let directories: Vec<String> = Vec::new();
        Lines::File(File { name, size, directories })
    };
}


pub(crate) fn part1() -> i32 {
    let input = include_str!("actual.txt");
    let lines = input.lines().enumerate();

    let root = Directory { name: String::from("/"), files: vec![], parent: None };
    let mut directories: Vec<Directory> = vec![];
    let mut files: Vec<File> = vec![];
    let mut directory_stack: Vec<String> = vec![];
    let mut current_dir: Option<Directory> = Some(root.clone());
    directories.push(root);

    for (_idx, line) in lines {
        let parsed_line = parse_line(line);
        match parsed_line {
            Lines::Cd(dir) => {
                if dir.name == ".." {
                    directory_stack.pop();
                    continue;
                }
                current_dir = Some(dir.clone());
                directory_stack.push(dir.name);
            }
            Lines::Ls => {
                // let mut files: Vec<File> = vec![];
                // let mut last_directory = directories.last_mut().unwrap();
                // for line in input.lines().skip(idx + 1) {
                //     let parsed_line = parse_line(line);
                //     match parsed_line {
                //         Lines::File(mut file) => {
                //             file.directories = directory_stack.clone();
                //             files.push(file);
                //         }
                //         Lines::Dir(dir) => {
                //             last_directory.children.push(dir.clone());
                //             directories.push(dir);
                //         }
                //         _ => { break;}
                //     }
                // }
            }
            Lines::Dir(mut dir) => {
                dir.parent = Some(directory_stack.last().unwrap().clone());
                directories.push(dir);
            }
            Lines::File(file) => {
                let mut local_file = file.clone();
                local_file.directories = directory_stack.clone();
                if let Some(d) = &mut current_dir {
                    d.files.push(local_file.clone());
                }
                files.push(local_file);
            }
        }
    }

    // by dir instead
    // for file in files.clone() {
    //     let file_dirs = file.directories.clone();
    //     for dir in file_dirs {
    //         for directory in &mut directories {
    //             if directory.name == dir {
    //                 directory.files.push(file.clone());
    //                 break;
    //             }
    //         }
    //     }
    // }


    let mut sizes = vec![];
    for dir in directories.clone() {
        let mut size = 0;
        for file in dir.files {
            size += file.size;
        }
        sizes.push(size);
        println!("{}: {}", dir.name, size);
    }

    let result = sizes
        .into_iter()
        .filter(|&x| x <= 100000)
        .collect::<Vec<i32>>()
        .into_iter().sum();

    println!("{:?}", result);


    return result;
}