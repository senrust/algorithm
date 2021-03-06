use std::cmp::Ordering::{Less, Greater, Equal};
use std::mem::swap;


// A type implementing Indexed<K> that is used as value in a BinaryTree may be indexed by K,
// that is, lookup functions can take a key: K instead of the full value. This is necessary for
// implementing associative containers.
pub trait Indexed<K: ?Sized> {
    fn key(&self) -> &K;
}

impl<T> Indexed<T> for T where T: Ord {
    fn key(&self) -> &T { self }
}


type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    left: Link<T>,
    right: Link<T>
}

pub struct BinaryTree<T> {
    root: Link<T>
}


impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    // Get a reference to the link at which "key" is or should be located
    fn locate<K>(&self, key: &K) -> &Link<T> where T: Indexed<K>, K: Ord {
        let mut anchor = &self.root;
        while let Some(ref node) = *anchor {
            match node.value.key().cmp(&key) {
                Less => anchor = &node.left,
                Greater => anchor = &node.right,
                Equal => return anchor
            }
        }
        // No such entry, anchor is pointing to the insert position of value
        anchor
    } 


    fn locate_mut<K>(&mut self, key: &K) -> &mut Link<T> where T: Indexed<K>, K: Ord {
        let mut anchor = &mut self.root;
        loop {
            match anchor.as_ref() {
                Some(n) if key != n.value.key() => {
                    let node = anchor.as_mut().unwrap(); 
                    // let node = anchor.as_mut().unwrap();
                    if key < node.value.key() {
                        anchor =  &mut node.left 
                    } else {
                        anchor = &mut node.right 
                    }
                }
                _ => return anchor,
            }
        }
    }

    pub fn lookup<K>(&self, key: &K) -> Option<&T> where T: Indexed<K>, K: Ord {
        self.locate(key).as_ref().map(|node| &node.value)
    }

    pub fn insert(&mut self, value: T) -> bool where T: Ord {
        let anchor = self.locate_mut(&value);
        match *anchor {
            Some(_) => false,
            None    => {
                *anchor = Some(Box::new(Node { value: value, left: None, right: None }));
                true
            }
        }
    }

    pub fn delete<K>(&mut self, key: &K) where T: Indexed<K>, K: Ord {
        delete_node(self.locate_mut(key));
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { current: &self.root, stack: Vec::new() }
    }
}


// Returns the next in-order successor in a subtree
fn successor<T>(mut next: &mut Link<T>) -> &mut Link<T> {
    loop {
        match {next.as_ref()} {
            Some(node) if node.left.is_some() => next = &mut next.as_mut().unwrap().left,
            _ => {
                debug_assert!(next.is_some());
                return next;
            }
        }
    }
}


// Removes a node, either by simply discarding it if it is a leaf, or by swapping it with
// its inorder successor (which, in this case, is always in a leaf) and then deleting the leaf.
fn delete_node<T>(link: &mut Link<T>) {
    if let Some(mut  boxed_node) = link.take() {
        match (boxed_node.left.take(), boxed_node.right.take()) {
            (None, None) => (),
            (Some(left), None) => *link = Some(left),
            (None, Some(right)) => *link = Some(right),
            (Some(left), Some(right)) => {
                // take() followed by re-assignment looks like an awful hackjob, but appears
                // to be the only way to satisfy all cases in the match
                {
                    let node = &mut *boxed_node; // unbox
                    node.left = Some(left);
                    node.right = Some(right);
                    let next = successor(&mut node.right);
                    swap(&mut node.value, &mut next.as_mut().unwrap().value);
                    delete_node(next);
                }
                *link = Some(boxed_node);
            }
        }
    }
}

// Allow iterating over &tree
impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


pub struct Iter<'a, T: 'a> {
    current: &'a Link<T>,
    stack: Vec<&'a Node<T>>
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ref node) = *self.current {
            self.stack.push(node);
            self.current = &node.left;
        }
        self.stack.pop().map(|node| {
            self.current = &node.right;
            &node.value
        })
    }
}