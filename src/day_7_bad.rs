use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::read_lines;

pub fn run() {
    println!("\nDay 7");

    let node = Rc::new(RefCell::new(TreeNode::new_dir("/".to_string(), None)));
    if let Ok(lines) = read_lines("assets/day_7.txt") {
        for line in lines {
            let str = line.expect("line error");

            if str.starts_with("$") {
                let command = str
                    .trim_start_matches("$")
                    .trim_end_matches("$")
                    .trim();

                let mut command_split = command.split(" ");
                let command_name = command_split.next().unwrap();

                let command_args = command_split.next();
                let vec = command_split.collect::<Vec<&str>>();
                println!("COMMAND {} {:?} {:?}", command_name, command_args, vec);

                match command_name {
                    "cd" => {
                        let value = command_args.expect("command_args error");

                        let mut ref_mut = node.borrow_mut();
                        *ref_mut = ref_mut.clone().cd(value);
                        drop(ref_mut);
                    }
                    "ls" => {}
                    _ => {
                        println!("UNKNOWN COMMAND {} {:?}", command_name, vec);
                    }
                }
            } else if str.starts_with("dir") {
                let i = str.find(' ').expect("find error");
                let name = &str[i..].trim();

                let dir = TreeNode::new_dir(name.to_string(), Some(node.clone()));
                node.borrow_mut().add_child(dir);
            } else {
                let mut split = str.split(" ");

                let size = split.next().expect("split next error").trim().parse::<u32>().expect("parse size error");
                let name = split.next().expect("split next error").trim();
                let file = TreeNode::new_file(name.to_string(), size, Some(node.clone()));
                node.borrow_mut().add_child(file);
            }
        }
    }

    println!("node {:?}", node);
}

#[derive(Debug, Clone)]
struct TreeNode {
    name: String,
    is_dir: bool,
    children: Vec<TreeNodeRef>,
    size: u32,
    parent: Option<TreeNodeRef>,
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

impl TreeNode {
    fn new_dir_no_parent(name: String) -> TreeNode {
        TreeNode {
            name,
            is_dir: true,
            children: Vec::new(),
            size: 0,
            parent: None,
        }
    }

    fn new_dir(name: String, parent: Option<TreeNodeRef>) -> TreeNode {
        TreeNode {
            name,
            is_dir: true,
            children: Vec::new(),
            size: 0,
            parent,
        }
    }

    fn add_child(&mut self, child: TreeNode) {
        self.children.push(Rc::new(RefCell::new(child)));
    }

    fn cd(self, value: &str) -> TreeNode {
        let name = self.name.clone();
        if value == ".." {
            if let Some(parent) = &self.parent {
                return parent.borrow().clone();
            }
            panic!("node has no parent");
        }

        if name == value {
            return self.clone();
        }

        for node in &self.children {
            println!("COMPARING {:?} {:?}", node.borrow().name, value);
            if node.borrow().name == value {
                return node.borrow().clone();
            }
        }

        println!("not found in child {:?} \n{:?}", value, self);
        panic!("not found in child ");
    }

    fn new_file(name: String, size: u32, parent: Option<TreeNodeRef>) -> TreeNode {
        TreeNode {
            name,
            is_dir: false,
            children: vec![],
            size,
            parent,
        }
    }
}

pub fn run_old() {
    println!("\nDay 7");

    let mut node = Box::new(Node::new_dir("/".to_string(), None));

    if let Ok(lines) = read_lines("assets/day_7.txt") {
        for line in lines {
            let str = line.expect("line error");

            if str.starts_with("$") {
                let command = str
                    .trim_start_matches("$")
                    .trim_end_matches("$")
                    .trim();

                let mut command_split = command.split(" ");
                let command_name = command_split.next().unwrap();

                let command_args = command_split.next();
                let vec = command_split.collect::<Vec<&str>>();
                println!("COMMAND {} {:?} {:?}", command_name, command_args, vec);

                match command_name {
                    "cd" => {
                        let value = command_args.expect("command_args error");
                        node = node.cd(value);
                    }
                    "ls" => {}
                    _ => {
                        println!("UNKNOWN COMMAND {} {:?}", command_name, vec);
                    }
                }
            } else if str.starts_with("dir") {
                let i = str.find(' ').expect("find error");
                let name = &str[i..];
                let dir = Node::new_dir(name.to_string(), Some(node.clone()));
                node.add_child(dir);
            } else {
                let mut split = str.split(" ");

                let size = split.next().expect("split next error").parse::<u32>().expect("parse size error");
                let name = split.next().expect("split next error");
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    name: String,
    is_dir: bool,
    children: Vec<Node>,
    size: u32,
    parent: Option<Box<Node>>,
}

impl Node {
    fn new(name: String, size: u32) {}
    fn new_dir(name: String, parent: Option<Box<Node>>) -> Node {
        Node {
            name,
            is_dir: true,
            children: Vec::new(),
            size: 0,
            parent,
        }
    }
    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    fn cd(self, value: &str) -> Box<Node> {
        let name = self.name.clone();
        if value == ".." {
            return self.clone().parent.expect("node has no parent");
        }

        if name == value {
            return Box::new(self.clone());
        }

        let x = self.children.iter().find(|s| s.name == value).expect("not found in child");
        Box::new(x.clone())
    }
}