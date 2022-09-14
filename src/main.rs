#[derive(Debug, Clone)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    data: i32,
}

impl Node {
    fn new(value: i32) -> Option<Box<Self>> {
        Some(Box::new(Node {
            data: value,
            right: None,
            left: None,
        }))
    }
}

#[derive(Debug)]
struct BinaryTree {
    root: Option<Box<Node>>,
    size: i32,
}

impl BinaryTree {
    fn new() -> Self {
        BinaryTree {
            root: None,
            size: 0,
        }
    }

    fn insert(&mut self, data: i32) {
        let temp = Node::new(data);

        match self.root {
            None => self.root = temp,
            Some(_) => {
                let mut current = self.root.as_mut().unwrap();
                loop {
                    if data < current.data {
                        if current.left.is_none() {
                            current.left = temp;
                            break;
                        };

                        current = current.left.as_mut().unwrap();
                    } else {
                        if current.right.is_none() {
                            current.right = temp;
                            break;
                        };
                        current = current.right.as_mut().unwrap();
                    }
                }
            }
        }
        self.size += 1;
    }

    fn search(&self, value: i32) -> Option<Box<Node>> {
        match &self.root {
            None => return None,
            Some(node) => {
                let mut current = node;
                loop {
                    println!("{} ", &current.data);

                    if value < current.data {
                        current = current.left.as_ref().unwrap();
                    } else if value > current.data {
                        current = current.right.as_ref().unwrap();
                    } else {
                        return Some(current.clone());
                    }
                }
            }
        }
    }

    fn inorder_traversal(&self, root: &Option<Box<Node>>) {
        match &root {
            None => {}
            Some(node) => {
                self.inorder_traversal(&node.left);
                print!("{} ", node.data);
                self.inorder_traversal(&node.right);
            }
        }
    }
}

fn main() {
    let data = [34, 84, 15, 0, 16, 99, 79, 9, 88, 89, 18, 31, 39, 100, 101];

    let mut binary_tree = BinaryTree::new();
    for i in data {
        binary_tree.insert(i);
    }
    //println!("{:#?}", binary_tree);
    //binary_tree.inorder_traversal(&binary_tree.root);
    let node = binary_tree.search(200);
    println!("{:#?}", node);
}
