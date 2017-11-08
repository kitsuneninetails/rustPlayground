use std::io;

pub fn exercise1() {
    for i in 0..30 {
        match i {
            l if l % 3 == 0 => println!("Pling"),
            l if l % 5 == 0 => println!("Plang"),
            l if l % 7 == 0 => println!("Plong"),
            _ => println!("{}", i)
        }
    }
    println!("Hello!")
}

struct Foo<'a> {
    bar: &'a mut String,
}

impl<'a> Foo<'a> {
    fn mut_bar(&mut self) -> &mut String {
        &mut self.bar
    }
}
pub fn exercise2() {
    let mut s = "Hello".to_string();
    let mut f = Foo {
        bar: &mut s,
    };
    {
        *f.mut_bar() = "World".to_string();
    }

    println!("{}", f.bar);
}