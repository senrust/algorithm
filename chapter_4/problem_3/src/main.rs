use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node<T: Clone> {
    value: T,
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value: value, 
            prev: None,
            next: None,
        }))
    }
}

#[derive(Debug)]
struct LinkedList<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T: Clone> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None, 
            tail: None, 
            size: 0,
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn append(&mut self, value: T) {
        let node = Node::new(value);
        match self.tail.take() {
            // first append
            None => {
                self.head = Some(Rc::clone(&node));  
            }
            Some(oldtail) => {
                oldtail.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(Rc::downgrade(&oldtail));
            }
        };
        self.size += 1;
        self.tail = Some(Rc::clone(&node));
    }

    fn pop(&mut self) -> Option<T> {
        match self.tail.take(){
            Some(tail_node) => {
                if let Some(prev) = tail_node.borrow_mut().prev.take() {
                    let prev = prev.upgrade().unwrap();
                    prev.borrow_mut().next = None;
                    self.tail = Some(prev);
                } else {
                    self.head = None;
                }
                self.size -= 1;
                // must be exactly one strong reference
                let value = Rc::try_unwrap(tail_node).ok().unwrap().into_inner().value;
                Some(value)
            }
            None => None
        }
    }
    
    fn get(&self, index: usize) -> Option<T>{
        let mut searching_index = 0;
        if self.size < index {
            return None
        }

        let mut node = match self.head.as_ref() {
            Some(first_node) => {
                Rc::clone(first_node)
            }
            None => return None,
        };

        while searching_index != index{
            // 一度tmp_nodeで受け取らないといけない どうすればいいのか?
            // let node = match node.borrow().next
            // とするとnodeが借用中でコンパイルできない
            let tmp_node = match node.borrow().next {
                Some(ref next_node) => Rc::clone(&next_node),
                None => return None,
            };
            // nodeをnode.nextにする
            node = tmp_node;
            searching_index += 1;
        }
        let value = node.borrow().value.clone();
        // Some(node.borrow().value.clone())とするとエラー
        // どうすればいいのか?
        Some(value)        
    }
}

#[derive(Clone, Debug)]
struct Person{
    age: i32,
    name: String,
}

fn main() {
    let mut linked_person = LinkedList::<Person>::new();
    let adult = Person{age: 30, name: "adult".to_string()};
    linked_person.append(adult);
    let child = Person{age: 10, name: "child".to_string()};
    linked_person.append(child);
    println!("linked list len is {}", linked_person.len());
    println!("2nd Person name is {}", linked_person.get(1).unwrap().name);
    println!("2nd Person name is {}", linked_person.pop().unwrap().name);
    println!("linked list len is {}", linked_person.len());
    println!("2nd Person name is {:?}", linked_person);
}
