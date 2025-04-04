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

impl <T> Stack<T> {
    pub fn new() -> Self {
        Stack {head: Link::Tail}
    }

    pub fn push(&mut self, new: T) -> () {
        let newnode = Node {next: mem::replace(&mut self.head, Link::Tail), value: new};
        self.head = Link::Body(Box::new(newnode));
    }

    pub fn pop(&mut self) -> Option<T> {
        let me = mem::replace(&mut self.head,Link::Tail);
        return match me {
            Link::Tail => None,
            Link::Body(thing) => {
                let node= *thing;
                let itm = node.value;
                self.head = node.next;
                Some(itm)
            }
        };
    }
}

pub fn main() {
    let mut a = Stack::new();
    for i in 0..10 {
        a.push(i);
    }
    for i in 0..11 {
        println!("{:?}",a.pop());
    }
}