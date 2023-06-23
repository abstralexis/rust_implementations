use anyhow::{Result, anyhow};

pub struct LinearQueue<T: Clone> {
    size: usize,
    queue: Vec<Option<T>>,
}

impl<T: Clone> LinearQueue<T> {
    pub fn new(size: usize) -> Self {
        LinearQueue {
            size: size,
            queue: vec![None; size],
        }
    }

    pub fn enqueue(&mut self, item: T) -> Result<()> {
        if self.queue.len() == self.size {
            return Err(anyhow!("Queue exceeded size {}", &self.size));
        }

        self.queue.push(Some(item));

        Ok(())
    }

    pub fn dequeue(&mut self) -> Result<T> {
        match self.queue[0] {
            Some(item) => {
                self.queue.remove(0);
                Ok(item)
            },
            None => Err(anyhow!("Cannot dequeue from empty queue"))
        }
    }
}

// impl<T: Clone> ToString for LinearQueue<T> {
//     fn to_string(&self) -> String {
        
//     }
// }
