// use crate::List::{Cons, Nil};

fn main() {
    // example_box();
    example_pointing();
}


// #[warn(dead_code)]
// #[warn(unused_variables)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil
// }

// #[warn(unused_variables)]
// fn _example_list(){
//     let list= Cons(4, Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))));
// }

fn example_pointing(){
    let x=5;
    let y= Box::new(x);

    assert_eq!(5,x);
    assert_eq!(5, *y);
}


fn _example_box(){
    let b= Box::new(56);
    println!("b = {b}");
}


struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}

fn example_mybox(){
    let x=5;
    let y=MyBox::new(2);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}