pub struct BTree<T> {
    root: Option<Node<T>>,
}

impl<T> BTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Crate a new BTree with this tree as the left child and the other tree as the right child.
    fn implant_sx(self, new_root: T, other: BTree<T>) -> Self {
        let mut new_root = Node::new(new_root);
        new_root.left = self.root.map(Box::new);
        new_root.right = other.root.map(Box::new);
        Self {
            root: Some(new_root),
        }
    }

    /// Crate a new BTree with the other tree as the left child and this tree as the right child.
    fn implant_dx(self, new_root: T, other: BTree<T>) -> Self {
        let mut new_root = Node::new(new_root);
        new_root.left = other.root.map(Box::new);
        new_root.right = self.root.map(Box::new);
        Self {
            root: Some(new_root),
        }
    }

    fn mut_root(&mut self) -> Option<&mut Node<T>> {
        self.root.as_mut()
    }
}

struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn boxed(value: T) -> Box<Self> {
        Box::new(Self::new(value))
    }

    fn left(&self) -> Option<&Node<T>> {
        self.left.as_deref()
    }

    fn right(&self) -> Option<&Node<T>> {
        self.right.as_deref()
    }

    fn set_left(&mut self, node: Node<T>) {
        self.left = Some(Box::new(node));
    }

    fn set_right(&mut self, node: Node<T>) {
        self.right = Some(Box::new(node));
    }
}
