use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;
use anyhow::Result;

pub type NodeRef<T> = Rc<Box<Node<T>>>;

#[derive(Debug, Clone, Copy)]
pub struct TooManyChildren {}
impl Error for TooManyChildren {}
impl Display for TooManyChildren {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This Node has too mant children.")
    }
}

#[derive(Debug, Clone)]
pub struct BTree<T: Default> {
    pub root: Option<NodeRef<T>>,
}

impl<T: Default + Copy> BTree<T> {
    pub fn new() -> Self {
        Self { root: Some(Rc::new(Box::new(Node::new()))) }
    }

    pub fn traverse(self, path: &bool) -> Option<NodeRef<T>> {
        match self.root {
            Some(root) => match path {
                true => return root.children.1.clone(),
                false => return root.children.0.clone(),
            }, 
            None => return None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub item: T,
    pub parent: Option<NodeRef<T>>,
    pub children: (Option<NodeRef<T>>, Option<NodeRef<T>>),
}

impl<T: Default + Copy> Node<T> {
    pub fn new() -> Self {
        Self { item: T::default(), parent: None, children: (None, None) }
    }

    pub fn create_child(&mut self, path: &bool, value: &T) -> Result<()> {
                let mut child: Node<T> = Node::new();
                child.parent = Some(Rc::new(Box::new(*self)));
                child.item = *value;
                        
                match path {
                    true => match self.children {
                        (_, None) => self.children.1 = Some(Rc::new(Box::new(child))),
                        _ => return Err(TooManyChildren{}.into()),
                    }
                    false => match self.children {
                        (None, _) => self.children.0 = Some(Rc::new(Box::new(child))),
                        _ => return Err(TooManyChildren{}.into())
                    }
                }

        Ok(())
    }
} 