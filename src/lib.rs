// https://github.com/SimonSapin/rust-forest
// https://github.com/RazrFalcon/rctree/blob/master/src/lib.rs

mod macros;
#[allow(unused_imports)]
use crate::macros::*;

use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

/// Mutable reference.
type Link<T, U> = Rc<RefCell<Node<T, U>>>;
/// Weak mutable reference.
type WeakLink<T, U> = Weak<RefCell<Node<T, U>>>;

/// Partial Operator.
#[derive(Debug, Eq, PartialEq)]
pub enum PartialOp {
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    /// Compare to the smallest value of the available.
    Min,
    /// Compare to the biggest value of the available.
    Max,
    /// Compare to the median value of the available
    Median,
}

pub struct DT<T, U>(Link<T, U>)
where
    U: PartialEq + PartialOrd + Copy;

#[derive(std::fmt::Debug)]
struct Node<T, U>
where
    U: PartialEq + PartialOrd + Copy,
{
    children: Vec<Link<T, U>>,
    latest_parent: Option<WeakLink<T, U>>,
    latest_child: Option<Link<T, U>>,
    decision: Option<U>,
    data: T,
}

/// Cloning a 'Node' only increments a reference count. It does not copy the data.
impl<T, U> Clone for DT<T, U>
where
    U: PartialEq + PartialOrd + Copy,
{
    fn clone(&self) -> Self {
        DT(Rc::clone(&self.0))
    }
}

impl<T, U: PartialEq + PartialOrd> PartialEq for DT<T, U>
where
    U: PartialEq + PartialOrd + Copy,
{
    fn eq(&self, other: &DT<T, U>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

// If T has trait debug
impl<T: std::fmt::Debug, U: std::fmt::Debug> std::fmt::Debug for DT<T, U>
where
    U: PartialEq + PartialOrd + Copy,
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
    U: PartialEq + PartialOrd + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.borrow(), f)
    }
}

impl<T, U> DT<T, U>
where
    T: Clone,
    U: PartialEq + PartialOrd + Copy,
{
    /// Returns the data value.
    pub fn data_clone(&self) -> T {
        self.0.borrow().data.clone()
    }
}

impl<T, U> DT<T, U>
where
    T: Copy,
    U: PartialEq + PartialOrd + Copy,
{
    /// Returns the data value.
    pub fn data(&self) -> T {
        self.0.borrow().data
    }
}

impl<T, U> DT<T, U>
where
    U: PartialEq + PartialOrd + Copy,
{
    /// Initialize the decision tree.
    /// It is also possible to use `new`, but there is no reason to give the root any decisions.
    pub fn init(data: T) -> DT<T, U> {
        DT(Rc::new(RefCell::new(Node {
            children: Vec::new(),
            latest_parent: None,
            latest_child: None,
            decision: None,
            data: data,
        })))
    }

    /// Create new instance of a node.
    pub fn new(data: T, decision: U) -> DT<T, U> {
        DT(Rc::new(RefCell::new(Node {
            children: Vec::new(),
            latest_parent: None,
            latest_child: None,
            decision: Some(decision),
            data: data,
        })))
    }
    /// Returns the amount of children that node contains.
    pub fn len(&self) -> usize {
        self.0.borrow().children.len()
    }
    /// Returns the decision value.
    pub fn decision(&self) -> Option<U> {
        self.0.borrow().decision
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
    pub fn first(&self) -> Option<DT<T, U>> {
        self.child(0)
    }
    /// Returns a reference to the last child.
    ///
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutably borrowed.
    pub fn last(&self) -> Option<DT<T, U>> {
        self.child(self.len() - 1)
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
                Some(_) => self.latest_parent().unwrap().back(steps - 1),
                None => None,
            }
        } else {
            Some(self.clone())
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
    pub fn forward_first(&self, steps: usize) -> Option<DT<T, U>> {
        if steps > 0 {
            // Recursion
            match self.first() {
                Some(_) => self.first().unwrap().forward_first(steps - 1),
                None => None,
            }
        } else {
            Some(self.clone())
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
    pub fn forward_last(&self, steps: usize) -> Option<DT<T, U>> {
        if steps > 0 {
            // Recursion
            match self.last() {
                Some(_) => self.last().unwrap().forward_last(steps - 1),
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

pub struct Traverse<T, U>
where
    U: PartialEq + PartialOrd + Copy,
{
    current: Option<Link<T, U>>,
}

impl<T, U> Traverse<T, U>
where
    U: PartialEq + PartialOrd + Copy,
{
    /// Start node to traverse from.
    pub fn start(node: DT<T, U>) -> Traverse<T, U> {
        Traverse {
            current: Some(node.0),
        }
    }

    /// Traverse to next node based on its decision.
    ///
    /// If none of the operations is met, return `None`.
    pub fn traverse(&mut self, decision: U, partial_op: PartialOp) -> Option<DT<T, U>> {
        // If it is an superiority type or not.
        if matches!(
            partial_op,
            PartialOp::Min | PartialOp::Max | PartialOp::Median
        ) {
            match partial_op {
                PartialOp::Min => {
                    let link = &self.current.clone().unwrap();
                    let children = &link.borrow().children;
                    // Start min value
                    let mut min_node = &children[0];
                    for (i, child) in children.iter().enumerate() {
                        // Continue if decision is none
                        if child.borrow().decision.is_none() {
                            continue;
                        }
                        // Set the min node if the child is less than it
                        if min_node.borrow().decision.unwrap() > child.borrow().decision.unwrap() {
                            min_node = child;
                        }
                        // Return if on the last child
                        if i >= children.len() - 1 {
                            // If decision is less than smallest value
                            if decision < min_node.borrow().decision.unwrap() {
                                return Some(DT(min_node.clone()));
                            }
                        }
                    }
                }
                PartialOp::Max => {
                    let link = &self.current.clone().unwrap();
                    let children = &link.borrow().children;
                    // Start max value
                    let mut max_node = &children[0];
                    for (i, child) in children.iter().enumerate() {
                        // Continue if decision is none
                        if child.borrow().decision.is_none() {
                            continue;
                        }
                        // Set the max node if the child is greater than it
                        if max_node.borrow().decision.unwrap() < child.borrow().decision.unwrap() {
                            max_node = child;
                        }
                        // Return if on the last child
                        if i >= children.len() - 1 {
                            // If decision is greater than biggest value
                            if decision > max_node.borrow().decision.unwrap() {
                                return Some(DT(max_node.clone()));
                            }
                        }
                    }
                }
                PartialOp::Median => {
                    let link = &self.current.clone().unwrap();
                    let children = &mut link.borrow_mut().children;
                    // Sort based on decision values
                    children.sort_by(|a, b| {
                        a.borrow()
                            .decision
                            .unwrap()
                            .partial_cmp(&b.borrow().decision.unwrap())
                            .unwrap()
                    });
                    // Average value
                    let average_node = children[children.len() / 2].clone();
                    // If decision is greater than biggest value
                    if decision == average_node.borrow().decision.unwrap() {
                        return Some(DT(average_node.clone()));
                    }
                }
                _ => panic!("{:?} is not supported", partial_op),
            }
        } else {
            for child in self.current.clone().unwrap().borrow().children.iter() {
                let child_borrow = &child.borrow();
                // Continue if decision is none
                if child_borrow.decision.is_none() {
                    continue;
                }
                match partial_op {
                    PartialOp::Greater => {
                        if decision > child_borrow.decision.unwrap() {
                            self.current = Some(child.clone());
                            return Some(DT(child.clone()));
                        }
                    }
                    PartialOp::GreaterEqual => {
                        if decision >= child_borrow.decision.unwrap() {
                            self.current = Some(child.clone());
                            return Some(DT(child.clone()));
                        }
                    }
                    PartialOp::Less => {
                        if decision < child_borrow.decision.unwrap() {
                            self.current = Some(child.clone());
                            return Some(DT(child.clone()));
                        }
                    }
                    PartialOp::LessEqual => {
                        if decision <= child_borrow.decision.unwrap() {
                            self.current = Some(child.clone());
                            return Some(DT(child.clone()));
                        }
                    }
                    PartialOp::Equal => {
                        if decision == child_borrow.decision.unwrap() {
                            self.current = Some(child.clone());
                            return Some(DT(child.clone()));
                        }
                    }
                    _ => panic!("{:?} is not supported", partial_op),
                }
            }
        }
        None
    }
}
