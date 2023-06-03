//! This is a cons list implementation.

pub use std::error::Error;
pub use std::fmt;

/// This is the error thrown when attempting to traverse a `Cons` list, and the `Cons`
/// variant that the traversal function is being called on is `Cons::None`.
#[derive(Debug, Clone)]
pub struct EmptyConsError {}
impl Error for EmptyConsError {}
impl fmt::Display for EmptyConsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot traverse an empty Cons List.")
    }
}

/// # Usage
/// A cons list is constructed by
/// ```
/// let cons: Cons<T> = Cons::Node(T, Box::new(Cons::Node( ... )));
/// ```
///
/// This is a simple, singly linked list that consists of a pointer-value pair,
/// where a pointer to another `Cons` object is contained inside a `Box` as the
/// second argument to `Node`. This `Cons` object can be a `Node` or `None`,
/// indicating the end of the list.
#[derive(Debug, Clone, PartialEq)]
pub enum Cons<T>
where
    T: Clone + PartialEq,
{
    Node { val: T, ptr: Box<Cons<T>> },
    Nil,
}

// TODO get functionality for Cons List working. ATM it seems limited by Type Checker.
// /// This is limited by Type Checker because you can't use Struct Variants of Enums as Types.
// impl<T> Cons::Node<T>
// where
//     T: Clone + PartialEq,
// {
//     /// This function appends a value to the list.
//     pub fn append(&mut self, item: T) -> Result<(), EmptyConsError> {
//         // Set variables to represent the state of the current `Node`.
//         let mut value: T = &self[0];
//         let mut ptr: Box<Cons<T>> = &self[1];

//         // Hopefully a traversal?
//         while match *ptr {
//             Cons::Node(_, _) => true,
//             Cons::Nil => false,
//         } {
//             value = *ptr[0];
//             ptr = *ptr[1];
//         }

//         Ok(())
//     }
// }
