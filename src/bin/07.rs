use std::{cell::RefCell, collections::HashMap, rc::{Weak, Rc}};

#[derive(Debug)]
struct File {
    size: u64,
}

#[derive(Debug)]
struct DirInner {
    subdirs: HashMap<String, Dir>,
    files: HashMap<String, File>,
    parent: WeakDir,
}

#[derive(Debug, Clone)]
struct Dir(Rc<RefCell<DirInner>>);

#[derive(Debug, Clone)]
struct WeakDir(Weak<RefCell<DirInner>>);


impl Dir {
    fn new(parent: Option<&Self>) -> Self {
        let weak_parent = match parent {
            Some(parent) => {
                let rc_dir = &parent.0;
                WeakDir(Rc::downgrade(rc_dir))

            },
            None => WeakDir(Weak::new()),
        };

        Self(Rc::new(RefCell::new(DirInner {
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
        let dir = self.0.borrow();

        match dir_name {
            ".." => {
                let weak_parent = dir.parent.clone();
                let parent = weak_parent.0.upgrade().unwrap();

                Dir(parent)
            }
            _ => {
                let subdirs = &dir.subdirs;
                let subdir = subdirs.get(dir_name).unwrap().clone();
                subdir
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

fn create_fs(input: &str) -> Dir {
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

    fs
}

#[derive(Copy, Clone)]
enum DirSizeComps {
    LE,
    GE,
}

fn dir_search(fs: Dir, size: u64, comp: DirSizeComps) -> Vec<Dir> {
    let mut dirs: Vec<Dir> = Vec::new();

    fn search(dir_: Dir, dirs: &mut Vec<Dir>, size: u64, comp: DirSizeComps)
    {
        let dir_clone = dir_.clone();
        let comp_result = match comp {
            DirSizeComps::LE => dir_.size() <= size,
            DirSizeComps::GE => dir_.size() >= size,
        };

        if comp_result {
            dirs.push(dir_clone)
        }

        let dir_clone = dir_.clone();
        let dir_inner = dir_clone.0.borrow();
        for subdir in dir_inner.subdirs.values() {
            let subdir_clone = subdir.clone();
            search(subdir_clone, dirs, size, comp)
        }
    }

    search(fs, &mut dirs, size, comp);
    dirs

}

pub fn part_one(input: &str) -> Option<u64> {
    let fs = create_fs(input);

    let dirs_under_100k = dir_search(fs, 100_000, DirSizeComps::LE);

    let result: u64 = dirs_under_100k
        .iter()
        .map(
            |dir| dir.size()
        )
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let fs = create_fs(input);

    let size_left = 70_000_000 - fs.size();
    let size_req = 30_000_000 - size_left;

    let dirs_to_free_space = dir_search(fs, size_req, DirSizeComps::GE);

    let result: u64 = dirs_to_free_space
        .iter()
        .map(
            |dir| dir.size()
        )
        .min()
        .unwrap();

    Some(result)
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
        assert_eq!(part_two(&input), Some(24933642));
    }
}
