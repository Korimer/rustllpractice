use std::{mem};
#[derive(Debug)]
pub struct Stack<T> {
    head: Link<T>
}

#[derive(Debug)]
struct Node<T> {
    next: Link<T>,
    value: T
}

type Link<T> = Option<Box<Node<T>>>;

impl <T> Stack<T> {
    pub fn new() -> Self {
        Stack {head: None}
    }

    pub fn push(&mut self, new: T) -> () {
        let newnode = Node {next: mem::take(&mut self.head), value: new};
        self.head = Some(Box::new(newnode));
    }

    pub fn pop(&mut self) -> Option<T> {
        mem::take(&mut self.head).map(
            |itm| -> T {
                self.head = itm.next;
                itm.value
            }
        )
    }

    pub fn iter<'a>(&self) -> Iter<'a, T> {
        Iter {next: self.head.as_ref().map(|item| -> &Node<T> {item})}
    }    
}

impl <T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut current = mem::take(&mut self.head);
        while let Some(mut thing) = current {
            current = mem::take(&mut thing.next);
        }
    }
}

impl <T> Iterator for Stack<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}
impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some => {
                let 
            }
        }
    }
}

pub fn main() {
    let mut a = Stack::new();
    let mut b = Stack::new();
    let mut c = Stack::new();
    for i in 0..10 {
        a.push(i);
        b.push(i);
        c.push(i);
    }
    for _i in 0..11 {
        println!("{:?}",a.pop());
    }
    while let Some(num) = b.pop() {
        println!("{}",num);
    }
    drop(c);
}