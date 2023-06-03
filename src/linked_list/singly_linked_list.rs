//! This is a file for a singly linked list implementation.

//? The comments in this file should explain the theory behind this - I doubt anyone will
//? want to actually use this code, but it can be learned from.

// /// This just says hello from this file. Nothing special, mainly just a sanity check for me.
// pub fn hello_linked() {
//     println!("Hello Linked!");
// }

/// Implementation of a linked list. The implementation I found and genericised essentially
/// functions like a stack. This means that is has a `push` and a `pop` function that work
/// as expected for a stack.
/// # Example usage
/// ```
/// let list: List<i32> = List::new().push(10).push(20); // Push some values to a new list
/// let popped = match list.pop() {
///     None => panic!(),
///     Some(val) => val,
/// }; // Pops the value and handles the `Option`.
/// println!("{:?}", &popped); // Prints 20
/// ```
#[derive(Debug, Clone)]
pub struct List<T>
where
    T: Clone,
{
    head: Link<T>,
}

/// This is a type alias. It can either contain a pointer to a `Node`, or it can
/// contain nothing. This `Option` must be handled.
type Link<T> = Option<Box<Node<T>>>;

/// This is a Node. This is the part of the `List` that holds the values and the `Link`
/// to the next node. This `Link` is an `Option`, so it can either point to another
/// `Node`, or it can point to `None`. This results in the traditonal structure for a linked
/// list.
#[derive(Debug, Clone)]
struct Node<T>
where
    T: Clone,
{
    /// The value stored by the node
    elem: T,
    // The `Link` (pointer) to the next `Node` or `None`.
    next: Link<T>,
}

impl<T> List<T>
where
    T: Clone,
{
    /// Create a new `List` with an empty head.
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Pushes `val` onto the head of the `List`.
    /// # Example
    /// ```
    /// let list: List<i32> = List::new().push(10); // Creates a new `List` and pushes 10.
    /// ```
    pub fn push(&mut self, val: T) -> Self {
        // To push, we need to have a new node.
        let node = Node {
            // Assign the value as the element
            elem: val,
            // `.take()` on an `Option` makes it `None` and returns its previous value.
            // Here, we make the old head `None` and then make this new node's next `Link`
            // be the previous head.
            next: self.head.take(),
        };

        // Make that old head be a new `Link` - i.e. make it be the `Some` variant of
        // `Option` and `Box` our new node inside of it. We are just making the new head
        // point to a node that contains a value, and the old head. The head is like the
        // top of the stack.
        self.head = Some(Box::new(node));

        // Return a clone of self, so people can chain `push`es. Realistically not too
        // memory expensive as the excess is dropped immediately if not `pushed` to immediately.
        self.clone()
    }

    /// Removes and then returns the head of the list.
    /// # Example
    /// ```
    /// let list: List<i32> = List::new().push(10).push(20); // Push some values to a new `List`
    /// let popped = match list.pop() {
    ///     None => panic!(),
    ///     Some(val) => val,
    /// }; // Pops the value and handles the `Option`.
    /// println!("{:?}", &popped); // Prints 20
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        // Take the old head - make it `None` and then return the node out of it.
        match self.head.take() {
            None => None,
            // When `Some`, we must take the next node in the list and make that the head.
            // We then return the popped element to the caller.
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
