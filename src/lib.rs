// https://github.com/SimonSapin/rust-forest
// https://github.com/RazrFalcon/rctree/blob/master/src/lib.rs

use std::cell::{Cell, Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

thread_local! {static ID_COUNT: i32 = 0;}

/// Mutable reference.
type Link<T, U> = Rc<RefCell<Node<T, U>>>;
/// Weak mutable reference.
type WeakLink<T, U> = Weak<RefCell<Node<T, U>>>;

/// Return value if `Some`, else return `None`.
macro_rules! try_opt {
    ($expr: expr) => {
        match $expr {
            Some(value) => value,
            None => return None,
        }
    };
}

pub struct DT<T, U>(Link<T, U>)
where
    U: Eq + PartialEq;

#[derive(std::fmt::Debug)]
struct Node<T, U>
where
    U: Eq + PartialEq,
{
    children: Vec<Link<T, U>>,
    latest_parent: Option<WeakLink<T, U>>,
    latest_child: Option<Link<T, U>>,
    decision: U,
    data: T,
}

/// Cloning a 'Node' only increments a reference count. It does not copy the data.
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
    /// Create new instance of a node.
    pub fn new(data: T, decision: U) -> DT<T, U> {
        DT(Rc::new(RefCell::new(Node {
            children: Vec::new(),
            latest_parent: None,
            latest_child: None,
            decision: decision,
            data: data,
        })))
    }
    /// Returns the amount of children that node contains.
    pub fn len(&self) -> usize {
        self.0.borrow().children.len()
    }
    /// Returns a reference to the latest parent node.
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutably borrowed.
    pub fn latest_parent(&self) -> Option<DT<T, U>> {
        Some(DT(try_opt!(try_opt!(self
            .0
            .borrow()
            .latest_parent
            .as_ref())
        .upgrade())))
    }
    /// Returns a reference to the latest child node.
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutably borrowed.
    pub fn latest_child(&self) -> Option<DT<T, U>> {
        Some(DT(try_opt!(self.0.borrow().latest_child.as_ref()).clone()))
    }
    /// Returns a reference to a child by the index
    ///
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutably borrowed.
    pub fn child(&self, index: usize) -> Option<DT<T, U>> {
        Some(DT(try_opt!(self.0.borrow().children.get(index)).clone()))
    }
    /// Returns a reference to the first child.
    ///
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutably borrowed.
    pub fn first_child(&self) -> Option<DT<T, U>> {
        self.child(0)
    }
    /// Returns a reference to the last child.
    ///
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutably borrowed.
    pub fn last_child(&self) -> Option<DT<T, U>> {
        self.child(self.len())
    }
    /// Returns the root of the decision tree.
    ///
    /// O(N)
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutably borrowed.
    pub fn root(&self) -> DT<T, U> {
        // Recursion
        match self.latest_parent() {
            Some(_) => self.latest_parent().unwrap().root(),
            None => self.clone(),
        }
    }
    /// Returns a node based on the steps in the hierarchy.
    /// If it is unable to go back that far, return `None`.
    ///
    /// O(N)
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutably borrowed.
    pub fn back(&self, steps: usize) -> Option<DT<T, U>> {
        if steps > 0 {
            // Recursion
            match self.latest_parent() {
                Some(_) => self.latest_parent().unwrap().back(steps),
                None => None,
            }
        } else {
            Some(self.clone())
        }
    }
    /// Returns a shared reference to this node's data
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutably borrowed.
    pub fn borrow(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |v| &v.data)
    }
    /// Returns a unique/mutable reference to this node's data
    ///
    /// # Panics
    ///
    /// Panics if the node is currently borrowed.
    pub fn borrow_mut(&mut self) -> RefMut<T> {
        RefMut::map(self.0.borrow_mut(), |v| &mut v.data)
    }
    /// Append a new child to this node.
    ///
    /// # Panics
    ///
    /// Panics if the node tries to append to itself.
    pub fn append(&mut self, new_child: DT<T, U>) -> DT<T, U> {
        assert!(*self != new_child, "Not legal to append to itself");

        // Borrow the reference
        let mut self_borrow = self.0.borrow_mut();
        let mut new_child_borrow = new_child.0.borrow_mut();
        // Borrow a reference of the latest parent (this)
        new_child_borrow.latest_parent = Some(Rc::downgrade(&self.0));
        // Borrow a reference of the latest child (new_child)
        self_borrow.latest_child = Some(new_child.0.clone());

        self_borrow.children.push(new_child.0.clone());

        self.clone()
    }
    /// Returns true if it has any children.
    pub fn has_children(&self) -> bool {
        self.0.borrow().children.len() > 0
    }
    /// Returns true if it has any parents (not root).
    pub fn has_parent(&self) -> bool {
        self.latest_parent().is_some()
    }
    /// Returns true if it is the root (no parents).
    pub fn is_root(&self) -> bool {
        self.latest_parent().is_none()
    }
}
