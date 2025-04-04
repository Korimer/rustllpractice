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

impl <T> Default for Link<T> { fn default() -> Self { Link::Tail } }

impl <T> Stack<T> {
    pub fn new() -> Self {
        Stack {head: Link::Tail}
    }

    pub fn push(&mut self, new: T) -> () {
        let newnode = Node {next: mem::take(&mut self.head), value: new};
        self.head = Link::Body(Box::new(newnode));
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head,Link::Tail) {
            Link::Tail => None,
            Link::Body(thing) => {
                self.head = thing.next;
                Some(thing.value)
            }
        }
    }
}

impl <T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut current = mem::take(&mut self.head);
        while let Link::Body(mut thing) = current {
            current = mem::take(&mut thing.next);
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