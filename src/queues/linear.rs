//! This is a Linear Queue implementation.

use anyhow::{anyhow, Result};

/// This is an implementation of a Linear Queue.
#[derive(Debug, Clone)]
pub struct LinearQueue<T: Clone> {
    size: usize,
    queue: Vec<Option<T>>,
    next_index: usize, // This is almost like a null pointer. Whoops.
}

impl<T: Clone> LinearQueue<T> {
    /// Create a new linear queue with a max size of `size`.
    pub fn new(size: usize) -> Self {
        LinearQueue {
            size: size,
            queue: vec![None; size],
            next_index: 0_usize,
        }
    }

    /// Enqueue an item to `self`.
    pub fn enqueue(&mut self, item: T) -> Result<()> {
        // If the index for the next item exceeds the list size,
        // and an item is attempted to be enqueued, return an err.
        if self.next_index > self.size {
            return Err(anyhow!("Queue exceeded size {}", &self.size));
        }

        // Replace the item at the position of the next index, and
        // increment the next index.
        self.queue[self.next_index] = Some(item);
        self.next_index = self.next_index + 1;

        Ok(())
    }

    /// Dequeue an item from `self`.
    pub fn dequeue(&mut self) -> Result<T> {
        // Match the first item. If it is None, then
        // return an error.
        match self.queue[0].clone() {
            Some(item) => {
                // Push a None value to the end of the queue Vec
                self.queue.push(None);
                // Remove the first item in the queue
                self.queue.remove(0_usize);
                // Decrement the index
                self.next_index = self.next_index - 1;

                // Return the item.
                Ok(item)
            }
            None => Err(anyhow!("Cannot dequeue from empty queue")),
        }
    }
}
