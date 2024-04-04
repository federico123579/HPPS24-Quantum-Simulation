use std::{cell::RefCell, rc::Rc};

type NodeRef<I> = Rc<RefCell<Node<I>>>;

#[derive(Debug, Clone)]
struct Node<I> {
    item: I,
    next: Vec<NodeRef<I>>,
}

impl<I> Node<I> {
    fn new_rc(item: I) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            item,
            next: Vec::new(),
        }))
    }

    fn link_front(&mut self, next: Rc<RefCell<Self>>) {
        self.next.push(next);
    }
}

#[cfg(test)]
mod tests {
    use super::Node;

    #[test]
    fn test_simple_dag() {
        let a = Node::new_rc('a');
        let b = Node::new_rc('b');
        let c = Node::new_rc('c');
        let d = Node::new_rc('d');
        let e = Node::new_rc('e');

        a.borrow_mut().link_front(b.clone());
        a.borrow_mut().link_front(c.clone());
        a.borrow_mut().link_front(d.clone());
        b.borrow_mut().link_front(d.clone());
        c.borrow_mut().link_front(d.clone());
        c.borrow_mut().link_front(e.clone());
        d.borrow_mut().link_front(e.clone());

        println!("{:?}", a);
    }
}
