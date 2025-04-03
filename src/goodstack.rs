use std::mem;
#[derive(Debug)]
pub struct Stack<T> {
    head: Link<T>
}

#[derive(Debug)]
struct Node<T> {
    next: Link<T>,
    value: T
}

#[derive(Debug)]
enum Link<T> {
    Tail,
    Body(Box<Node<T>>)
}
impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {head: Link::Tail}
    }

    pub fn push(&mut self, new: T) -> () {
        let newnode = Node {next: mem::replace(&mut self.head, Link::Tail), value: new};
        self.head = Link::Body(Box::new(newnode));

    }
}

pub fn main() {
    let mut a = Stack::new();
    a.push(1);
    println!("lol")
}