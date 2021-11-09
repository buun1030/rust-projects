enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
// We can see that the Rc<List> in a has an initial reference count of 1; 
// then each time we call clone, the count goes up by 1. When c goes out of scope, 
// the count goes down by 1. We don’t have to call a function to decrease 
// the reference count like we have to call Rc::clone to increase the reference count: 
// the implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope.

// What we can’t see in this example is that when b and then a go out of scope at the end of main, 
// the count is then 0, and the Rc<List> is cleaned up completely at that point. Using Rc<T> allows 
// a single value to have multiple owners, and the count ensures that the value remains valid 
// as long as any of the owners still exist.