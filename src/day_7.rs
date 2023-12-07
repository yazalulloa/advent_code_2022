use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::utils::read_lines;

pub fn run() {
    println!("\nDay 7");

    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();

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
                // println!("COMMAND {} {:?} {:?}", command_name, command_args, vec);

                match command_name {
                    "cd" => {
                        let value = command_args.expect("command_args error").trim();
                        match value {
                            "/" => {
                                //  println!("nothing")
                            }
                            ".." => {
                                let parent = node.borrow().parent.clone().unwrap();
                                node = parent;
                            }
                            _ => {
                                let child = node.borrow_mut().children.iter().find(|child| {
                                    child.borrow().name == value
                                }).expect("child not found").clone();
                                node = child;
                            }
                        }
                    }
                    "ls" => {}
                    _ => {
                        println!("UNKNOWN COMMAND {} {:?}", command_name, vec);
                    }
                }
            } else if str.starts_with("dir") {
                let i = str.find(' ').expect("find error");
                let name = &str[i..].trim();
                let dir = Node::new_dir(name.to_string(), Some(node.clone()));
                node.borrow_mut().children.push(dir);
            } else {
                let mut split = str.split(" ");

                let size = split.next().expect("split next error").trim().parse::<usize>().expect("parse size error");
                let name = split.next().expect("split next error").trim();

                let file = Node::new_file(name.to_string(), size, Some(node.clone()));
                node.borrow_mut().children.push(file);
            }
        }
    }

    //println!("{:#?}", PrettyNode(&root));

    let sum_part1 = all_dirs(root.clone())
        .map(|d| d.borrow().total_size())
        .filter(|&s| s <= 100_000)
        /* .inspect(|s| {
             dbg!(s);
         })*/
        .sum::<u64>();

    dbg!(sum_part1);

    let total_space = 70000000_u64;
    let used_space = root.borrow().total_size();
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let removed_dir_size = all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s >= minimum_space_to_free)
        /*.inspect(|s| {
            dbg!(s);
        })*/
        .min();

    dbg!(removed_dir_size);
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start = if self.is_dir { "DIR" } else { "FILE" };
        let name = format!("{start} {}", &self.name);
        f.debug_struct(name.as_str())
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Default, Clone)]
struct Node {
    name: String,
    size: usize,
    is_dir: bool,
    children: Vec<NodeHandle>,
    parent: Option<NodeHandle>,
}

impl Node {
    fn new_dir(name: String, parent: Option<NodeHandle>) -> NodeHandle {
        let node = Node {
            name,
            size: 0,
            is_dir: true,
            children: Vec::new(),
            parent,
        };

        Rc::new(RefCell::new(node))
    }

    fn new_file(name: String, size: usize, parent: Option<NodeHandle>) -> NodeHandle {
        let node = Node {
            name,
            size,
            is_dir: false,
            children: vec![],
            parent,
        };

        Rc::new(RefCell::new(node))
    }

    fn is_dir(&self) -> bool {
        self.is_dir
    }

    fn total_size(&self) -> u64 {
        self.children
            .iter()
            .map(|child| child.borrow().total_size())
            .sum::<u64>()
            + self.size as u64
    }
}

type NodeHandle = Rc<RefCell<Node>>;

struct PrettyNode<'a>(&'a NodeHandle);

impl<'a> fmt::Debug for PrettyNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = self.0.borrow();
        if this.size == 0 {
            writeln!(f, "{} (dir)", this.name)?;
        } else {
            writeln!(f, "{} (file, size={})", this.name, this.size)?;
        }

        for (child) in &this.children {
            // not very efficient at all, but shrug

            for (index, line) in format!("{:?}", PrettyNode(child)).lines().enumerate() {
                if index == 0 {
                    writeln!(f, " {line}")?;
                } else {
                    writeln!(f, "  {line}")?;
                }
            }
        }
        Ok(())
    }
}

fn all_dirs(node: NodeHandle) -> Box<dyn Iterator<Item=NodeHandle>> {
    // clippy is wrong and should feel bad
    #[allow(clippy::needless_collect)]
        let children = node.borrow().children.iter().cloned().collect::<Vec<_>>();

    Box::new(
        std::iter::once(node).chain(
            children
                .into_iter()
                .filter_map(|c| {
                    if c.borrow().is_dir() {
                        Some(all_dirs(c))
                    } else {
                        None
                    }
                })
                .flatten(),
        ),
    )
}