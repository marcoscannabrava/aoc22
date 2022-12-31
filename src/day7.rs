use crate::helpers::read_file;
use std::collections::HashMap;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct FileSystem(HashMap<PathBuf, Vec<File>>);

impl FileSystem {
    /// Parses terminal commands and outputs into a HashMap
    fn build(mut self, terminal_output: &str) -> self::FileSystem {
        let cd_cmd = "$ cd";
        let ls_cmd = "$ ls";
        let dir_prefix = "dir ";

        let mut curr_dir = PathBuf::from("/");
        self.0.insert(curr_dir.clone(), vec![]);

        for l in terminal_output.lines() {
            if l.starts_with(cd_cmd) {
                match l.replace(cd_cmd, "").trim() {
                    "/" => curr_dir = PathBuf::from("/"),
                    ".." => {
                        curr_dir.pop();
                    }
                    dir_name => {
                        curr_dir.push(dir_name);
                    }
                }
            } else if l.starts_with(ls_cmd) {
                continue;
            } else if l.starts_with(dir_prefix) {
                let dir_name = l.replace(dir_prefix, "").trim().to_owned();
                let mut new_dir = curr_dir.clone();
                new_dir.push(dir_name);
                self.0.insert(new_dir, vec![]);
            } else if l.len() > 0 {
                let (size, name) = l.split_at(l.find(' ').unwrap());
                self.0.get_mut(&curr_dir).unwrap().push(File {
                    name: name.trim().to_string(),
                    size: size.parse::<usize>().unwrap(),
                });
            }
        }
        self
    }

    fn sum_files_size(&self, dir: &PathBuf) -> usize {
        match self.0.get(dir) {
            Some(dir) => dir.iter().map(|f| f.size).sum::<usize>(),
            None => 0,
        }
    }

    fn dir_size(&self, dir: &PathBuf) -> usize {
        let mut size: usize = 0;
        self.0.keys().filter(|k| k.starts_with(dir)).for_each(|k| {
            size += self.sum_files_size(k);
        });
        size
    }
}

pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day7.txt");

    let fs = FileSystem(HashMap::new()).build(&contents);
    let folders = fs.0.keys();
    let result1: usize = folders.clone()
        .map(|k| {
            let size = fs.dir_size(k);
            return if size < 100000 { size } else { 0 };
        })
        .sum();

    let available_mem = 70000000;
    let total_required_mem = 30000000;
    let used_mem = fs.dir_size(&PathBuf::from("/"));
    let delta_required_mem = total_required_mem - (available_mem - used_mem);
    let dir_sizes = folders.map(|k| fs.dir_size(k));
    let min_directory_size_greater_than_delta = dir_sizes
        .filter(|s| *s > delta_required_mem)
        .min()
        .unwrap();
    let result2 = min_directory_size_greater_than_delta;

    return (result1.to_string(), result2.to_string());
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use crate::day7::FileSystem;

    const TEST_INPUT: &str = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn build() {
        let mut fs: FileSystem = FileSystem(HashMap::new());
        fs = fs.build(TEST_INPUT);
        println!("FileSystem");
        for (k, v) in &fs.0 {
            println!("{}: {:?}", k.to_str().unwrap(), v);
        }
        let ae_dir = fs.0.get(&PathBuf::from("/a/e")).unwrap();
        assert_eq!(ae_dir[0].name, "i".to_string());
        assert_eq!(ae_dir[0].size, 584);
    }
}
