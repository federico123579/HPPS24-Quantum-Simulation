use std::rc::Rc;

struct Node<I> {
    item: I,
    next: Option<Rc<Node<I>>>,
}

impl<I> Node<I> {
    fn new_rc(item: I) -> Rc<Self> {
        Rc::new(Node { item, next: None })
    }

    fn chain_front(self: Rc<Self>, item: I) -> Rc<Self> {
        let new_node = Node {
            item,
            next: Some(self),
        };
        Rc::new(new_node)
    }
}
