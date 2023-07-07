//! This is a basic implementation of a stack.

use anyhow;

/// This is a Stack - it's just a wrapper around a Vec so that
/// it is a zero cost abstraction while also being able to 
/// restrict how it is used.
#[derive(Debug)]
pub struct Stack<T: Clone + Copy> {
    stack: Vec<T>,
    height: usize,
}

impl<T: Clone + Copy> Stack<T> {
    /// Make a new stack with height of `height`
    pub fn new(height: &usize) -> Self {
        Stack {
            stack: Vec::new(),
            height: *height,
        }
    }

    /// Push an item `item` to the stack, returning an error
    /// if the stack is full.
    pub fn push(&mut self, item: &T) -> anyhow::Result<()> {
        match self.stack.len() == self.height {
            true => Err(
                anyhow::anyhow!("Stack is full.")
            ),
            false => {
                self.stack.push(*item);
                Ok(())
            }
        }
    }

    /// Pop the top item from the stack and return a `Result`
    /// that Errors if the stack is empty, and if not returns
    /// that item. 
    pub fn pop(&mut self) -> anyhow::Result<T> {
        // This basically just turns the regular `Vec` behaviour
        // of returning an `Option` into returning a `Result`.
        match self.stack.pop() {
            None => Err(anyhow::anyhow!("Stack is empty")),
            Some(item) => Ok(item),
        }
    }

    /// Reimplementation of the `.last()` method with a new name.
    /// That's literally it.
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    /// Simple as innit
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Returns if the stack is full or not
    pub fn is_full(&self) -> bool {
        self.stack.len() == self.height
    }
}

