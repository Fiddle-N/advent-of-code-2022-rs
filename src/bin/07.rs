use std::{cell::RefCell, collections::HashMap, rc::{Weak, Rc}};

use itertools::Itertools;

#[derive(Debug)]
struct File {
    size: u64,
}

#[derive(Debug)]
struct Dir_ {
    subdirs: HashMap<String, Dir>,
    files: HashMap<String, File>,
    parent: WeakDir,
}

#[derive(Debug, Clone)]
struct Dir(Rc<RefCell<Dir_>>);

#[derive(Debug, Clone)]
struct WeakDir(Weak<RefCell<Dir_>>);


impl Dir {
    fn new(parent: Option<&Self>) -> Self {
        let weak_parent = match parent {
            Some(parent) => {
                let rc_dir = &(*parent).0;
                WeakDir(Rc::downgrade(rc_dir))

            },
            None => WeakDir(Weak::new()),
        };

        Self(Rc::new(RefCell::new(Dir_ {
            subdirs: HashMap::new(), 
            files: HashMap::new(),
            parent: weak_parent,
        })))
    }

    fn mkdir(&self, name: &str) {
        let mut dir = self.0.borrow_mut();
        dir.subdirs.insert(name.to_string(), Dir::new(Some(self)));
    }

    fn touch(&self, name: &str, size: u64) {
        let mut dir = self.0.borrow_mut();
        dir.files.insert(name.to_string(), File { size });
    }

    fn cd(&self, dir_name: &str) -> Dir {
        let mut dir = self.0.borrow_mut();

        match dir_name {
            ".." => {
                let parent = &dir.parent;
                let parent = parent.clone();
                let parent = parent.0.upgrade().unwrap();

                Dir(parent)
            }
            _ => {
                let subdirs = &dir.subdirs;
                let result = subdirs.get(dir_name);

                let result2 = result.unwrap();

                let result3 = result2.clone();      // performs underlying Rc clone
                result3
            }
        }
    }

    fn size(&self) -> u64 {
        let dir = self.0.borrow();
        let dir_sizes: u64 = dir
            .subdirs
            .values()
            .map(
                |dir|
                {
                    // let dir_ = dir.0.borrow();
                    dir.size()
                }
            )
            .sum();
        
        let file_sizes: u64 = dir
            .files
            .values()
            .map(
                |file| file.size
            )
            .sum();

        dir_sizes + file_sizes
        
    }

}

fn dir_search(fs: Dir) -> Vec<Dir> {
    let mut dirs: Vec<Dir> = Vec::new();
    let max_size = 100000;

    fn search(dir_: Dir, max_size: u64, dirs: &mut Vec<Dir>)
        {
            let dir_clone = dir_.clone();
            if dir_.size() < max_size {
                dirs.push(dir_clone)
            }

            let dir_clone = dir_.clone();
            let dir_2 = dir_clone.0.borrow();
            for dir_3 in dir_2.subdirs.values() {
                let dir_clone = dir_3.clone();
                search(dir_clone, max_size, dirs)
            }
        };

    search(fs, max_size, &mut dirs);
    dirs

}

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = input.lines();

    let fs;

    match input.next() {
        Some("$ cd /") => { fs = Dir::new(None) },
        _ => panic!("Unexpected input")
    }

    let mut curr_dir = fs.clone();
    let mut ls = false;

    for line in input {
        
        match line
            .split_ascii_whitespace()
            .collect::<Vec<_>>() 
            [..]           
        {
            ["$", "ls"] 
                => ls = true,

            ["$", "cd", dir_name]
                => {
                    ls = false;
                    curr_dir = curr_dir.cd(dir_name);
                }

            ["dir", dir_name] if ls == true
                => curr_dir.mkdir(dir_name),

            [size, file_name] 
                if ls == true
                => {
                    let size: u64 = size.parse().expect("Unexpected command");
                    curr_dir.touch(file_name, size);
                },

            _ => panic!("Unexpected command"),

        }
    }

    let dirs_under_100k = dir_search(fs);

    let result: u64 = dirs_under_100k
        .iter()
        .map(
            |dir| dir.size()
        )
        .sum();

    Some(result)

}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
