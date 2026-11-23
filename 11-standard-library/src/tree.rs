use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Узел дерева: дети владеются сильными ссылками, родитель — слабой.
#[derive(Debug)]
pub struct Node {
    value: i32,
    parent: RefCell<Option<Weak<Node>>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn new(value: i32) -> Rc<Self> {
        Rc::new(Self {
            value,
            parent: RefCell::default(),
            children: RefCell::default(),
        })
    }

    fn set_parent(&self, parent: Rc<Node>) {
        *self.parent.borrow_mut() = Some(Rc::downgrade(&parent));
    }

    pub fn add_child(self: &Rc<Self>, child: Rc<Node>) {
        child.set_parent(Rc::clone(self));
        self.children.borrow_mut().push(child);
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn child_values(&self) -> Vec<i32> {
        self.children
            .borrow()
            .iter()
            .map(|child| child.value)
            .collect()
    }

    pub fn has_parent(&self) -> bool {
        self.parent
            .borrow()
            .as_ref()
            .is_some_and(|parent| parent.strong_count() > 0)
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!(
            "Dropping node with value {} and {} children",
            self.value,
            self.children.borrow().len()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    use std::rc::Rc;

    #[test]
    fn weak_parent_does_not_create_a_cycle() {
        let tree = Node::new(1);
        let first = Node::new(3);
        let second = Node::new(5);
        tree.add_child(Rc::clone(&first));
        tree.add_child(Rc::clone(&second));

        assert_eq!(tree.child_values(), [3, 5]);
        assert!(first.has_parent());
        assert_eq!(Rc::strong_count(&tree), 1);

        drop(tree);
        assert!(!first.has_parent());
        assert!(!second.has_parent());
    }
}
