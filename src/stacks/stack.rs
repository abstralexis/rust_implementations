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
    /// if the stack is full
    pub fn push(&mut self, item: &T) -> anyhow::Result<()> {
        match self.stack.len() == self.height {
            true => Err(
                anyhow::anyhow!("Stack is full")
            ),
            false => {
                self.stack.push(*item);
                Ok(())
            }
        }
    }
}

