use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct Node<I> {
    item: I,
    ingoing: Vec<Rc<RefCell<Node<I>>>>,
    outgoing: Vec<Rc<RefCell<Node<I>>>>,
}

impl<I> Node<I> {
    fn new_rc(item: I) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            item,
            ingoing: Vec::new(),
            outgoing: Vec::new(),
        }))
    }

    fn add_outgoing(&mut self, next: Rc<RefCell<Self>>) {
        self.outgoing.push(next);
    }

    fn add_ingoing(&mut self, prev: Rc<RefCell<Self>>) {
        self.ingoing.push(prev);
    }

    fn get_outgoing(&self) -> &[Rc<RefCell<Self>>] {
        &self.outgoing
    }

    fn get_ingoing(&self) -> &[Rc<RefCell<Self>>] {
        &self.ingoing
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

        a.borrow_mut().add_outgoing(b.clone());
        a.borrow_mut().add_outgoing(c.clone());
        a.borrow_mut().add_outgoing(d.clone());
        b.borrow_mut().add_outgoing(d.clone());
        c.borrow_mut().add_outgoing(d.clone());
        c.borrow_mut().add_outgoing(e.clone());
        d.borrow_mut().add_outgoing(e.clone());

        println!("{:?}", a);
    }
}
