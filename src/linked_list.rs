use std::fmt::{Debug};
use std::boxed::Box;

struct ListItem<T: Clone + Debug> {
    pub item: T,
    pub tail: Option<Box<ListItem<T>>>,
}

impl<T: Clone + Debug> ListItem<T> {
    pub fn new(item: T) -> Self {
        ListItem {
            item,
            tail: None,
        }
    }

    pub fn new_tail(&mut self, item: T) {
        match &mut self.tail {
            Some(ref mut t) => {
                t.new_tail(item);
            },
            None => {
                self.tail = Some(Box::new(ListItem::new(item)));
            }
        }
    }

    pub fn follow_tail(&self) -> &ListItem<T> {
        match self.tail.as_ref() {
            Some(t) => t.follow_tail(),
            None => &self
        }
    }

    pub fn pop_tail(&mut self) {
        if let Some(ref mut t) = self.tail {
            if let None = t.tail.as_ref() {
                self.tail = None
            } else {
                t.pop_tail()
            }
        }
    }
}

struct LinkedList<T: Clone + Debug> {
    item_head: Option<Box<ListItem<T>>>
}

impl<T: Clone + Debug> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            item_head: None,
        }
    }

    pub fn head(&self) -> Option<T> {
        self.item_head.as_ref().map(|e| e.item.clone())
    }

    pub fn tail(&self) -> Option<T> {
        self.item_head.as_ref().map(|e| e.follow_tail().item.clone())
    }

    pub fn item(&self, pos: u32) -> Option<T> {
        let mut val = self.item_head.as_ref();
        for _ in 0..pos {
            val = val.and_then(|ref t| t.tail.as_ref());
        }
        val.map(|ref i| i.item.clone())
    }

    pub fn push_back(&mut self, item: T) {
        match &mut self.item_head {
            Some(ref mut h) => {
                h.new_tail(item);
            },
            None => {
                self.item_head = Some(Box::new(ListItem::new(item)));
            }
        }
    }

    pub fn pop_back(&mut self) {
        if let Some(ref mut h) = &mut self.item_head {
            h.pop_tail();
        }
    }
}


pub fn main() {
    let mut l = LinkedList::new();

    l.push_back(3u32);
    l.push_back(5u32);
    l.push_back(7u32);
    l.push_back(9u32);

    println!("Head={:?}", l.head());
    println!("Tail={:?}", l.tail());
    println!("0={:?}", l.item(0));
    println!("1={:?}", l.item(1));
    println!("2={:?}", l.item(2));
    println!("3={:?}", l.item(3));

    l.pop_back();

    println!("Head={:?}", l.head());
    println!("Tail={:?}", l.tail());
    println!("0={:?}", l.item(0));
    println!("1={:?}", l.item(1));
    println!("3={:?}", l.item(3));

}
