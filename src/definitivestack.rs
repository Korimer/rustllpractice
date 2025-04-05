use std::mem;
#[derive(Debug)]
pub struct Stack<'a, T> {
    head: Link<T>,
    cur: &'a Link<T>
}

#[derive(Debug)]
enum Link<T> {
    Tail,
    Body(Box<Node<T>>)
}
impl <'a, T> Default for Link<T> { fn default() -> Self { Link::Tail } }

#[derive(Debug)]
struct Node<T> {
    next: Link<T>,
    value: T
}

impl <'a, T> Stack<'a, T> {
    pub fn new() -> Self {
        Stack {head: Link::Tail, cur: &Link::Tail}
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

impl <'a, T> Drop for Stack<'a, T> {
    fn drop(&mut self) {
        let mut current = mem::take(&mut self.head);
        while let Link::Body(mut thing) = current {
            current = mem::take(&mut thing.next);
        }
    }
}

impl <'a, T> Iterator for Stack<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
impl <'a, T> Iterator for &Stack<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match mem::replace(&mut &self.cur, &&Link::Tail) {
            Link::Tail => None,
            Link::Body(thing) => {
                mem::replace(&mut &self.cur, &&thing.next);
                Some(&thing.value)
            }
        }
    }
}

pub fn main() {
    let mut a = Stack::new();
    let mut b = Stack::new();
    let mut c = Stack::new();
    let mut d = Stack::new();
    for i in 0..10 {
        a.push(i);
        b.push(i);
        c.push(i);
        d.push(i);
    }
    for _i in 0..11 {
        println!("{:?}",a.pop());
    }
    while let Some(num) = b.pop() {
        println!("{}",num);
    }
    drop(c);
    for itm in &d {
        println!("{itm}");
    }
    let _e = d;
}