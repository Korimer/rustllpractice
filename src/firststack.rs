use std::mem;

#[derive(Debug)]
pub enum ListItem<T> {
    None,
    Next(T,Box<ListItem<T>>)
}

#[derive(Debug)]

struct ListHolder<T> {
    head: ListItem<T>
}
impl <T> ListHolder<T> {
    pub fn new() -> Self {
        ListHolder { head: ListItem::None }
    }

    pub fn add (&mut self, item: T) -> () {
        let temp = Box::new(
            mem::replace(&mut self.head,ListItem::None)
        );
        self.head = ListItem::Next(item, temp);
    }

/*
    pub fn pop(&mut self) -> Option<T> {
        let poppeditem;
        let next;
        match &self.head {
            ListItem::None => {
                poppeditem = None;
                next = &ListItem::None;
            },
            ListItem::Next(itm,listitem) => {
                poppeditem = Some(*itm);
                next = &*listitem;
            }
        }
        let temp: ListItem<T> = mem::replace(&mut self.head, *next);
        return poppeditem;
    }
*/// Shared reference misery......
}

pub fn main() {
    let mut a = ListHolder::<i32>::new();
    for i in 0..10 {
        a.add(i);
    }
    println!("{:?}",a)
}
