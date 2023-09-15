
enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::{Cons, Nil};

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }


}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { // &Self::Target or &T
        &self.0
    }
}



fn main() {
    // let b = Box::new(5);
    // println!("b = {}", b);


    //let list = Cons(1, Cons(2, Cons(3, Nil)));
    let _list = Cons(1, Box::new(
                        Cons(2, Box::new(
                            Cons(3, Box::new(Nil))))));
    
    
    let x = 5;
    // let y = Box::new(x);
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *(y.deref()));


}
