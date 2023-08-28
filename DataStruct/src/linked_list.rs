use std::ptr::NonNull;
use std::fmt::{self, Display, Formatter};

struct Node<T> {
    val:T,
    next:Option<NonNull<Node<T>>>,
    prev:Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t:T)->Self{
        Self {
            val: t,
            next: None,
            prev: None,
        }
    }
}

pub struct LinkedList<T> {
    start:Option<NonNull<Node<T>>>,
    end:Option<NonNull<Node<T>>>,
    len:usize
}

impl<T> Display for LinkedList<T>
    where
        T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
    where T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

impl<T> LinkedList<T>{
    pub fn new()->Self{
        Self {
            start: None,
            end: None,
            len: 0,
        }
    }
    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.len += 1;
    }
    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }
    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

}



