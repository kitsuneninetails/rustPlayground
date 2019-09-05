use std::fmt::{Debug};
use std::boxed::Box;

struct LinkedList<T: Clone + Debug> {
    head: Option<T>,
    tail: Option<Box<LinkedList<T>>>,
}

impl<T: Clone + Debug> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn head(&self) -> Option<T> {
        self.head.clone()
    }

    pub fn tail(&self) -> Option<T> {
        if let Some(ref t) = self.tail {
            t.tail()
        } else {
            self.head.clone()
        }
    }

    pub fn item(&self, pos: u32) -> Option<T> {
        if pos == 0 {
            self.head()
        } else {
            if let Some(ref t) = self.tail {
                t.item(pos - 1)
            } else {
                None
            }
        }
    }

    pub fn push_front(&mut self, item: T) {
        self.tail = Some(Box::new(LinkedList {
            head: self.head.take(),
            tail: self.tail.take()
        }));
        self.head = Some(item);
    }

    pub fn pop_front(&mut self) {
        if let Some(ref mut t) = &mut self.tail {
            self.head = t.head.take();
            self.tail = t.tail.take();
        } else {
            self.head = None
        }
    }

    pub fn push_back(&mut self, item: T) {
        if let Some(ref mut h) = &mut self.head {
            if let Some(ref mut t) = &mut self.tail {
                t.push_back(item);
            } else {
                let mut new_tail = LinkedList::new();
                new_tail.head = Some(item);
                self.tail = Some(Box::new(new_tail));
            }
        } else {
            self.head = Some(item);
            self.tail = None;
        }
    }

    pub fn pop_back(&mut self) {
        if let Some(ref mut t) = &mut self.tail {
            t.pop_back();
            if let None = t.head.as_ref() {
                self.tail = None;
            }
        } else {
            self.head = None;
        }
    }

}

impl<T: Clone + Debug + ToString> ToString for LinkedList<T> {
    fn to_string(&self) -> String {
       format!("{}{}",
               match self.head.as_ref() {
                   Some(h) => format!("{}", h.to_string()),
                   None => "".to_string(),
               },
               match self.tail.as_ref() {
                   Some(t) => format!(", {}", t.to_string()),
                   None => "".to_string(),
               })
    }
}

pub fn main() {
    let mut l = LinkedList::new();

    l.push_back(3u32);
    l.push_back(5u32);
    l.push_back(7u32);

    println!("[{}]", l.to_string());

    l.push_back(9u32);

    println!("[{}]", l.to_string());

    l.pop_back();

    println!("[{}]", l.to_string());

    l.push_front(1u32);

    println!("[{}]", l.to_string());

    l.pop_front();

    println!("[{}]", l.to_string());

}
