//! This is my main file for this project. Whenever it runs, it should demo all of
//! the implementations that I have done. It would probably be cleaner to use a lib.rs
//! at some point, and execute each manually through the terminal, however I opted to
//! go with this for simplicity. I also know it better.

mod linked_list;
mod subfolder;

use linked_list::{cons::*, singly_linked_list::*};
use subfolder::hello::hello_world;

fn main() {
    println!("Hello, world!");
    hello_world();
    // hello_linked(); // Uncommenting would require uncommenting in the file too.

    let consi32: Cons<i32> = Cons::Node {
        val: 10,
        ptr: Box::new(Cons::Node {
            val: 20,
            ptr: Box::new(Cons::Nil),
        }),
    };
    println!("ConsList | {:?}", &consi32);

    let mut singly_linked: List<i32> = List::new().push(10).push(20).push(30);
    let popped: i32 = match singly_linked.pop() {
        None => panic!(),
        Some(val) => val,
    };
    println!("Singly Linked List (Stack) | {:?}", &singly_linked);
    println!("Popped Value | {:?}", &popped);
}
