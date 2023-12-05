// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("counter after creating a = {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("counter after creating b = {}", Rc::strong_count(&a));
//     {
//         let c = Cons(3, Rc::clone(&a));
//         println!("counter after creating c = {}", Rc::strong_count(&a));
//     }
//     println!(
//         "counter after c goes out of scope = {}",
//         Rc::strong_count(&a)
//     );
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
// 没有做出来
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("counter after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("counter after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(3, Box::new(Rc::clone(&a)));
        println!("counter after creating c = {}", Rc::strong_count(&a));
    }
    println!(
        "counter after c goes out of scope = {}",
        Rc::strong_count(&a)
    );
}
