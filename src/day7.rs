/*
--- Day 7: No Space Left On Device ---
You can hear birds chirping and raindrops hitting leaves as the expedition proceeds. Occasionally, you can even hear much louder sounds in the distance; how big do the animals get out here, anyway?

The device the Elves gave you has problems with more than just its communication system. You try to run a system update:

$ system-update --please --pretty-please-with-sugar-on-top
Error: No space left on device
Perhaps you can delete some files to make space for the update?

You browse around the filesystem to assess the situation and save the resulting terminal output (your puzzle input). For example:

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
7214296 k
The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files). The outermost directory is called /. You can navigate around the filesystem, moving into or out of directories and listing the contents of the directory you're currently in.

Within the terminal output, lines that begin with $ are commands you executed, very much like some modern computers:

cd means change directory. This changes which directory is the current directory, but the specific result depends on the argument:
cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
cd / switches the current directory to the outermost directory, /.
ls means list. It prints out all of the files and directories immediately contained by the current directory:
123 abc means that the current directory contains a file named abc with size 123.
dir xyz means that the current directory contains a directory named xyz.
Given the commands and output in the example above, you can determine that the filesystem looks visually like this:

- / (dir)
- a (dir)
- e (dir)
- i (file, size=584)
- f (file, size=29116)
- g (file, size=2557)
- h.lst (file, size=62596)
- b.txt (file, size=14848514)
- c.dat (file, size=8504156)
- d (dir)
- j (file, size=4060174)
- d.log (file, size=8033020)
- d.ext (file, size=5626152)
- k (file, size=7214296)
Here, there are four directories: / (the outermost directory), a and d (which are in /), and e (which is in a). These directories also contain files of various sizes.

Since the disk is full, your first step should probably be to find directories that are good candidates for deletion. To do this, you need to determine the total size of each directory. The total size of a directory is the sum of the sizes of the files it contains, directly or indirectly. (Directories themselves do not count as having any intrinsic size.)

The total sizes of the directories above can be found as follows:

The total size of directory e is 584 because it contains a single file i of size 584 and no other directories.
The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
Directory d has total size 24933642.
As the outermost directory, / contains every file. Its total size is 48381165, the sum of the size of every file.
To begin, find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes. In the example above, these directories are a and e; the sum of their total sizes is 95437 (94853 + 584). (As in this example, this process can count files more than once!)

Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?

--- Part Two ---
Now, you're ready to choose a directory to delete.

The total disk space available to the filesystem is 70000000. To run the update, you need unused space of at least 30000000. You need to find a directory you can delete that will free up enough space to run the update.

In the example above, the total size of the outermost directory (and thus the total amount of used space) is 48381165; this means that the size of the unused space must currently be 21618835, which isn't quite the 30000000 required by the update. Therefore, the update still requires a directory with total size of at least 8381165 to be deleted before it can run.

To achieve this, you have the following options:

Delete directory e, which would increase unused space by 584.
Delete directory a, which would increase unused space by 94853.
Delete directory d, which would increase unused space by 24933642.
Delete directory /, which would increase unused space by 48381165.
Directories e and a are both too small; deleting them would not free up enough space. However, directories d and / are both big enough! Between these, choose the smallest: d, increasing unused space by 24933642.

Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update. What is the total size of that directory?

*/
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
