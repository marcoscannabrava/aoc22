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
*/
use crate::helpers::read_file;
use regex::Regex;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type NodeRef<'a> = Rc<RefCell<Node<'a>>>;

#[derive(Debug)]
struct Node<'a> {
    tree: Box<Tree<'a>>,
    location: String,
    name: String,
    size: usize,
    parent: Option<NodeRef<'a>>,
    children: Option<Vec<NodeRef<'a>>>,
}

impl<'a> Node<'a> {
    fn append_child(&mut self, mut child: Node<'a>) -> NodeRef {
        child.location = format!("{}/{}", self.location, child.name);
        child.parent = self.tree.get_node(self.location.as_str());
        let node_ref = self.tree.add_node(child);
        match self.children {
            Some(ref mut children) => children.push(node_ref),
            None => self.children = Some(vec![node_ref]),
        }
        node_ref
    }
    fn remove_child(&mut self, child: NodeRef) {
        let child_node = child.borrow();
        match self.children {
            Some(ref mut children) => {
                let index = children
                    .iter()
                    .position(|x| x.borrow().name == child_node.name)
                    .unwrap();
                children.remove(index);
                self.tree.all_nodes.remove(child_node.location.as_str());
                if child_node.children.is_some() {
                    let grandchildren = child_node.children.unwrap().clone();
                    for grandchild in grandchildren {
                        child_node.remove_child(grandchild);
                    }
                }
            }
            None => (),
        }
    }
}

#[derive(Debug)]
struct Tree<'a> {
    all_nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> Tree<'a> {
    fn get_node(&self, location: &str) -> Option<NodeRef> {
        match self.all_nodes.get(location) {
            Some(node) => Some(Rc::new(RefCell::new(*node))),
            None => None,
        }
    }
    fn add_node(self: Box<Self>, node: Node) -> NodeRef {
        node.tree = self;
        self.all_nodes.insert(node.location.as_str(), node);
        Rc::new(RefCell::new(node))
    }
}

/// Parses terminal commands and outputs into a tree of Nodes.
/// Returns a tree: HashMap of Nodes.
fn build_tree(terminal_output: &str) -> Tree {
    let cd_cmd_re = Regex::new(r"$ cd (.*)").unwrap();
    let ls_cmd_re = Regex::new(r"$ ls").unwrap();
    let file_re = Regex::new(r"(\d+) (.*)").unwrap();
    let dir_re = Regex::new(r"dir (.*)").unwrap();

    let mut tree: Box<Tree>;
    let root: Node = Node {
        tree: Box::new(tree),
        name: String::from("/"),
        location: String::from("/"),
        size: 0,
        parent: None,
        children: None,
    };
    tree.add_node(root);
    
    let mut curr_node = tree.get_node("/").unwrap();

    for l in terminal_output.lines() {
        if cd_cmd_re.is_match(l) {
            let captures = cd_cmd_re.captures(l).unwrap();
            let dir_name = captures.get(1).unwrap().as_str();
            if dir_name == ".." {
                curr_node = curr_node.borrow_mut().parent.unwrap();
            }
        } else if file_re.is_match(l) {
            let captures = file_re.captures(l).unwrap();
            let size = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            curr_node.borrow_mut().append_child(Node {
                tree: tree,
                name: captures.get(2).unwrap().as_str().to_owned(),
                size: size,
                parent: None,
                location: "".to_owned(),
                children: None,
            });
        } else if dir_re.is_match(l) {
            let captures = dir_re.captures(l).unwrap();
            let dir_name = captures.get(1).unwrap().as_str();
            curr_node.borrow_mut().append_child(Node {
                tree: tree,
                name: String::from(dir_name),
                size: 0,
                children: None,
                parent: None,
                location: "".to_owned(),
            });
        }
    }
    return tree;
}

/// 1. Build tree representation of filesystem
/// 2. Find all directories with size <= 100000
/// 3. Sum the sizes of those directories
pub fn solution() -> (String, String) {
    let contents = read_file("/inputs/day7.txt");
    let result1: usize = 0;
    let result2: usize = 0;

    return (result1.to_string(), result2.to_string());
}

#[cfg(test)]
mod tests {
    use crate::day7;

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
    fn build_tree() {
        let nodes = day7::build_tree(TEST_INPUT);
        println!("nodes: {:?}", nodes);
    }
}
