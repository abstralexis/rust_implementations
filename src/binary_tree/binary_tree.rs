use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct InvalidPathError;
impl Error for InvalidPathError {}
impl Display for InvalidPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid path - is the path too long to traverse this tree?"
        )
    }
}

#[derive(Debug)]
pub struct HeadlessError;
impl Error for HeadlessError {}
impl Display for HeadlessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tree is headless.")
    }
}

#[derive(Debug, Clone)]
pub struct Tree<T>
where
    T: Clone,
{
    head: Branch<T>,
}

type Branch<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
pub struct Node<T> {
    value: Option<T>,
    parent: Branch<T>,
    children: (Branch<T>, Branch<T>),
}

impl<T> Tree<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            head: Some(Box::new(Node {
                value: None,
                parent: None,
                children: (None, None),
            })),
        }
    }

    // pub fn traverse(&mut self, path: &Vec<bool>) -> Result<&mut Node<T>, Box::<dyn Error>> {
    //     let mut curr = match &mut self.head {
    //         Some(c) => c,
    //         None => return Err(Box::new(HeadlessError)),
    //     };

    //     path.iter()
    //         .map(
    //             |traversal|
    //             Ok(
    //                 match traversal {
    //                     true => match curr.clone().children.1 {
    //                         Some(mut child) => curr = &mut child,
    //                         None => return Err(Box::new(InvalidPathError)),
    //                     },
    //                     false => match curr.clone().children.0 {
    //                         Some(mut child) => curr = &mut child,
    //                         None => return Err(Box::new(InvalidPathError))
    //                     },
    //                 }
    //             )
    //         );

    //     Ok(&mut curr)
    // }
}
