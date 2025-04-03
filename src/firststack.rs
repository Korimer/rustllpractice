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

}

pub fn main() {
    let mut a = ListHolder::<i32>::new();
    a.add(1);
    a.add(2);

    println!("{:?}",a);
}
