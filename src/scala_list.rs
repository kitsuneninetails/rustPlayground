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

    pub fn push_front(self, item: T) -> LinkedList<T> {
        LinkedList {
            head: Some(item),
            tail: Some(Box::new(LinkedList {
                head: self.head,
                tail: self.tail
            }))
        }
    }

    pub fn pop_front(self) -> LinkedList<T> {
        if let Some(t) = self.tail {
            LinkedList {
                head: t.head,
                tail: t.tail
            }
        } else {
            LinkedList::new()
        }
    }

    pub fn push_back(self, item: T) -> LinkedList<T> {
        if let Some(t) = self.tail {
            LinkedList {
                head: self.head,
                tail: Some(Box::new(t.push_back(item))),
            }
        } else {
            if let Some(h) = self.head {
                LinkedList {
                    head: Some(h),
                    tail: Some(Box::new(LinkedList {
                        head: Some(item),
                        tail: None
                    }))
                }
            } else {
                LinkedList {
                    head: Some(item),
                    tail: None
                }
            }

        }
    }

    pub fn pop_back(self) -> LinkedList<T> {
        if let Some(t) = self.tail {
            if let Some(_) = t.tail {
                LinkedList {
                    head: self.head,
                    tail: Some(Box::new(t.pop_back()))
                }
            } else {
                LinkedList {
                    head: self.head,
                    tail: None,
                }
            }
        } else {
            LinkedList::new()
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
    let l = LinkedList::new();

    let l = l.push_back(3u32);
    let l = l.push_back(5u32);
    let l = l.push_back(7u32);

    println!("[{}]", l.to_string());

    let l = l.push_back(9u32);

    println!("[{}]", l.to_string());

    let l = l.pop_back();

    println!("[{}]", l.to_string());

    let l = l.push_front(1u32);

    println!("[{}]", l.to_string());

    let l = l.pop_front();

    println!("[{}]", l.to_string());

}
