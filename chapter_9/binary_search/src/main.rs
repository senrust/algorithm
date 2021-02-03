#[derive(Debug)]
struct BinaryTree {
    value: Option<i32>,
    left_child: Option<Box<BinaryTree>>,
    right_child: Option<Box<BinaryTree>>,
}

impl BinaryTree {
    fn create() -> Self{
        BinaryTree {
            value: None,
            left_child: None,
            right_child: None,
        }
    }

    fn new(value: i32) -> Self{
        BinaryTree {
            value: Some(value),
            left_child: None,
            right_child: None,
        }
    }

    fn insert(&mut self, insert_value: i32) {
        let value = match self.value {
            Some(value) => value,
            None => {
                self.value = Some(insert_value);
                return;
            }
        };

        if insert_value < value {
            match self.left_child.as_mut() {
                Some(left_node) => {
                    left_node.insert(insert_value);
                },
                None => {
                    let left_child = Box::new(BinaryTree::new(insert_value));
                    self.left_child = Some(left_child);
                }
            }
        } else if value < insert_value {
            match self.right_child.as_mut() {
                Some(right_node) =>{
                    right_node.insert(insert_value);
                },
                None => {
                    let right_child = Box::new(BinaryTree::new(insert_value));
                    self.right_child = Some(right_child);
                } 
            }
        }
    }

    fn preorder_walk(&self) {
        match self.value {
            Some(value) => print!("{} ", value),
            None => {},
        }
        match self.left_child.as_ref() {
            Some(left_node) => left_node.preorder_walk(),
            None => {},
        }
        match self.right_child.as_ref() {
            Some(right_node) => right_node.preorder_walk(),
            None => {},
        }
    } 

    fn inorder_walk(&self) {
        match self.left_child.as_ref() {
            Some(left_node) => left_node.inorder_walk(),
            None => {},
        }
        match self.value {
            Some(value) => print!("{} ", value),
            None => {},
        }
        match self.right_child.as_ref() {
            Some(right_node) => right_node.inorder_walk(),
            None => {},
        }
    }

    fn outorder_walk(&self) {
        match self.left_child.as_ref() {
            Some(left_node) => left_node.outorder_walk(),
            None => {},
        }
        match self.right_child.as_ref() {
            Some(right_node) => right_node.outorder_walk(),
            None => {},
        }
        match self.value {
            Some(value) => print!("{} ", value),
            None => {},
        }
    }

    fn find(&self, key: i32) -> bool{
        match self.value {
            Some(value) => {
                if key == value {
                    return true
                }
            }
            None => return false
        }
        
        let left_result = match self.left_child.as_ref() {
            Some(left_node) => left_node.find(key),
            None => {false},
        };

        let right_result = match self.right_child.as_ref() {
            Some(right_node) => right_node.find(key),
            None => {false},
        };

        if left_result | right_result {
            true
        } else {
            false
        }
    }

    fn test(&mut self) {
        let node_a = &mut self.left_child;
        let node_b = &mut self.right_child;
        match node_a {
            &mut Some(ref mut xx) => {},
            None => {},
        }

        match node_b {
            Some(ref mut xx) => {},
            None => {},
        }
    }
}



fn main() {
    let input_size: usize = input::read_number();
    let mut binary_tree = BinaryTree::create();
    for _ in 0..input_size {
        let input_text: Vec<String> = input::read_numline();
        if input_text[0] == "insert" {
            let insert_value = input_text[1].parse::<i32>().ok().unwrap();
            binary_tree.insert(insert_value);
        }
        if input_text[0] == "find" {
            let key_value = input_text[1].parse::<i32>().ok().unwrap();
            let result = binary_tree.find(key_value);
            if result {
                println!("key {} was found!", key_value);
            } else {
                println!("key {} was not found...", key_value);
            }
        }
    }

    binary_tree.preorder_walk();
    print!("\n");
    binary_tree.inorder_walk();
    print!("\n");
    binary_tree.outorder_walk();
    print!("\n");
}
