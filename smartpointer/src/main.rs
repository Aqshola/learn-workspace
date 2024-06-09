// use crate::List::{Cons, Nil};

use std::rc::Rc;

fn main() {
    // example_box();
    // example_pointing();

    // example_drop();
    // example_recursive();
    example_pointing_reference();
}

#[allow(dead_code)]
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

#[allow(dead_code)]
struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}
#[allow(dead_code)]
#[allow(unused_variables)]
fn example_mybox(){
    let x=5;
    let y=MyBox::new(2);

    assert_eq!(5, x);
}


#[allow(dead_code)]
#[derive(Debug)]
enum List{
    Cons(i32, Box<List>),
    Nil,
}

#[allow(dead_code)]
fn example_recursive(){
    let listing=List::Cons(2, Box::new(List::Cons(2, Box::new(List::Nil))));

    println!("{:?}",listing);
}


#[allow(dead_code)]
struct CustomSmartPointer{
    data:String
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping pointer with data `{}`", self.data)
    }
}
#[allow(dead_code)]
#[allow(unused_variables)]
fn example_drop(){
    let c= CustomSmartPointer{
        data:String::from("Lalatina")
    };

    drop(c);

    let d= CustomSmartPointer{
        data:String::from("Darkness")
    };

    println!("CustomSmartPointer Created");
}

#[derive(Debug)]
enum NewList{
    Cons(i32, Rc<NewList>),
    Nil
}

fn example_pointing_reference(){
    
    let a= Rc::new(NewList::Cons(2, Rc::new(NewList::Nil))) ;

    let b= NewList::Cons(5, Rc::clone(&a));
    let c= NewList::Cons(15, Rc::clone(&a));

    println!("{:?},{:?},{:?}",a,b,c);
    

}