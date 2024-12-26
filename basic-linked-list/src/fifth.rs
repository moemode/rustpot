use std::mem;

pub struct List<T> {
    head: Link<T>,
    tail: Option<&mut Node<T>>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {head: None, tail: None}
    }

    pub fn push(&mut self, elem: T) {
        let new_tail = Box::new(Node { elem, next: None });
        let old_tail = std::mem::replace(&mut self.tail, Some(new_tail));
        match old_tail {
            Some(mut old_tail) => {
                old_tail.next = self.tail;
            }
            None => {
                self.head = Some(new_tail);
            }
        }
    }
}