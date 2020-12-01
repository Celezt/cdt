// https://github.com/RazrFalcon/rctree/blob/master/src/lib.rs

use std::cell::{Cell, Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

type Link<T, U> = Rc<RefCell<Node<T, U>>>;

pub struct DT<T, U>(Link<T, U>)
where
    U: Eq + PartialEq;

#[derive(std::fmt::Debug)]
struct Node<T, U>
where
    U: Eq + PartialEq,
{
    children: Vec<Link<T, U>>,
    decision: U,
    data: T,
}

impl<T, U: Eq> Clone for DT<T, U> {
    fn clone(&self) -> Self {
        DT(Rc::clone(&self.0))
    }
}

impl<T, U: Eq> PartialEq for DT<T, U> {
    fn eq(&self, other: &DT<T, U>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

// If T has trait debug
impl<T: std::fmt::Debug, U: std::fmt::Debug> std::fmt::Debug for DT<T, U>
where
    U: Eq + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let self_borrow = self.0.borrow();
        f.debug_tuple("Node")
            .field(&self_borrow.data)
            .field(&self_borrow.decision)
            .finish()
    }
}

impl<T: std::fmt::Display, U> std::fmt::Display for DT<T, U>
where
    U: Eq + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.borrow(), f)
    }
}

impl<T, U> DT<T, U>
where
    U: Eq + PartialEq,
{
    /// Create new instance of a node
    pub fn new(data: T, decision: U) -> DT<T, U> {
        DT(Rc::new(RefCell::new(Node {
            children: Vec::new(),
            decision: decision,
            data: data,
        })))
    }
    pub fn child_len(&self) -> usize {
        self.0.borrow().children.len()
    }
    /// Borrow an immutable reference
    pub fn borrow(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |v| &v.data)
    }
    /// Borrow an mutable reference
    pub fn borrow_mut(&mut self) -> RefMut<T> {
        RefMut::map(self.0.borrow_mut(), |v| &mut v.data)
    }
    /// Append a new child to this node.
    ///
    /// # Panics
    ///
    /// Panics if the node tries to append to itself.
    pub fn append(&mut self, new_child: DT<T, U>) {
        assert!(*self != new_child, "Not legal to append to itself");

        // Borrow the reference
        let mut self_borrow = self.0.borrow_mut();
        /* {
            let mut new_child_borrow = new_child.0.borrow_mut();
        } */

        self_borrow.children.push(new_child.0);

        //new_child_borrow
    }
    pub fn remove(&mut self) {}
}
