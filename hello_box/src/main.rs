// use crate::List::{Cons, Nil};

// enum List {
//     Cons(i32, List),
//     Nil,
// }

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // drop(c);
    // println!("CustomSmartPointer dropped before the end of main.");

    // let m = MyBox::new(String::from("Rust"));

    // hello("Rust");
    // hello(&m);
    // hello(&(*m)[..]);

    // let x = 5;
    // let y = MyBox::new(x);
    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let x = 5;
    // let y = &x;
    // let y = Box::new(x);
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    // println!("x = {}, y = {}", x, y);

    // let b = Box::new(5);
    // println!("b = {}", b);

    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
