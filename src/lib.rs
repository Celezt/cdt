// https://github.com/SimonSapin/rust-forest
// https://github.com/RazrFalcon/rctree/blob/master/src/lib.rs

mod macros;
#[allow(unused_imports)]
use crate::macros::*;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Mutable reference.
type Link<'a, T, U> = Rc<RefCell<Node<'a, T, U>>>;
/// Weak mutable reference.
type WeakLink<'a, T, U> = Weak<RefCell<Node<'a, T, U>>>;
/// Mutable reference to an hash map.
type HashLink<'a, T, U> = Rc<RefCell<std::collections::HashMap<&'a str, WeakLink<'a, T, U>>>>;

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

pub struct DT<'a, T, U>(Link<'a, T, U>)
where
    U: PartialEq + PartialOrd + Copy;

#[derive(std::fmt::Debug)]
struct Node<'a, T, U>
where
    U: PartialEq + PartialOrd + Copy,
{
    id: &'a str,
    children: Vec<Link<'a, T, U>>,
    latest_parent: Option<WeakLink<'a, T, U>>,
    latest_child: Option<Link<'a, T, U>>,
    decision: Option<U>,
    data: Option<T>,
    hash: HashLink<'a, T, U>,
}

/// Cloning a 'Node' only increments a reference count. It does not copy the data.
impl<'a, T, U> Clone for DT<'a, T, U>
where
    U: PartialEq + PartialOrd + Copy,
{
    fn clone(&self) -> Self {
        DT(Rc::clone(&self.0))
    }
}

impl<'a, T, U> PartialEq for DT<'a, T, U>
where
    U: PartialEq + PartialOrd + Copy,
{
    fn eq(&self, other: &DT<'a, T, U>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

// If T has trait debug
impl<'a, T, U> std::fmt::Debug for DT<'a, T, U>
where
    T: std::fmt::Debug,
    U: PartialEq + PartialOrd + Copy + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let self_borrow = self.0.borrow();
        f.debug_tuple("Node")
            .field(&self_borrow.data)
            .field(&self_borrow.decision)
            .finish()
    }
}

impl<'a, T, U> std::fmt::Display for DT<'a, T, U>
where
    T: std::fmt::Display,
    U: PartialEq + PartialOrd + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl<'a, T, U> DT<'a, T, U>
where
    T: Clone,
    U: PartialEq + PartialOrd + Copy,
{
    /// Returns the data value.
    pub fn data_clone(&self) -> T {
        self.0.borrow().data.clone().unwrap()
    }
}

impl<'a, T, U> DT<'a, T, U>
where
    T: Copy,
    U: PartialEq + PartialOrd + Copy,
{
    /// Returns the data value.
    pub fn data(&self) -> T {
        self.0.borrow().data.unwrap()
    }
}

impl<'a, T, U> DT<'a, T, U>
where
    U: PartialEq + PartialOrd + Copy,
{
    /// Create new instance of a node.
    fn new(
        id: &'a str,
        data: Option<T>,
        decision: Option<U>,
        hash: HashLink<'a, T, U>,
    ) -> DT<'a, T, U> {
        DT(Rc::new(RefCell::new(Node {
            id: id,
            children: Vec::new(),
            latest_parent: None,
            latest_child: None,
            decision: decision,
            data: data,
            hash: hash,
        })))
    }
    /// Initialize the decision tree.
    /// It is also possible to use `new`, but there is no reason to give the root any decisions.
    pub fn init() -> DT<'a, T, U> {
        // Initialize the hash map
        let hash = Rc::new(RefCell::new(std::collections::HashMap::new()));
        // Create new decision tree
        let dt: DT<'a, T, U> = DT::new("root", None, None, hash.clone());
        // insert the new decision tree into the hash map
        hash.borrow_mut()
            .insert("root", Rc::downgrade(&dt.0).clone());
        dt
    }
    /// Append a new child to this node.
    ///
    /// # Panics
    ///
    /// Panics if the node tries to append to itself.
    pub fn append(&mut self, id: &'a str, data: T, decision: U) -> DT<'a, T, U> {
        let new_child = DT::new(id, Some(data), Some(decision), self.0.borrow().hash.clone());
        // Insert id
        self.0
            .borrow()
            .hash
            .borrow_mut()
            .insert(id, Rc::downgrade(&new_child.0).clone());

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
    pub fn latest_parent(&self) -> Option<DT<'a, T, U>> {
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
    pub fn latest_child(&self) -> Option<DT<'a, T, U>> {
        Some(DT(try_opt!(self.0.borrow().latest_child.as_ref()).clone()))
    }
    /// Returns a reference to a child by the index
    ///
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutably borrowed.
    pub fn child(&self, index: usize) -> Option<DT<'a, T, U>> {
        Some(DT(try_opt!(self.0.borrow().children.get(index)).clone()))
    }
    /// Returns a reference to the first child.
    ///
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutably borrowed.
    pub fn first(&self) -> Option<DT<'a, T, U>> {
        self.child(0)
    }
    /// Returns a reference to the last child.
    ///
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutably borrowed.
    pub fn last(&self) -> Option<DT<'a, T, U>> {
        self.child(self.len() - 1)
    }
    /// Returns the root of the decision tree.
    ///
    /// O(N)
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutably borrowed.
    pub fn root(&self) -> Option<DT<'a, T, U>> {
        self.find("root")
    }
    /// Returns a node based on the steps in the hierarchy.
    /// If it is unable to go back that far, return `None`.
    ///
    /// O(N)
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutably borrowed.
    pub fn back(&self, steps: usize) -> Option<DT<'a, T, U>> {
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
    pub fn find(&self, find_id: &'a str) -> Option<DT<'a, T, U>> {
        match self.root().unwrap().0.borrow().hash.borrow().get(find_id) {
            Some(ref x) => Some(DT(try_opt!(x.upgrade()))),
            None => None,
        }
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

pub struct Traverse<'a, T, U>
where
    U: PartialEq + PartialOrd + Copy,
{
    current: Option<Link<'a, T, U>>,
}

impl<'a, T, U> Traverse<'a, T, U>
where
    U: PartialEq + PartialOrd + Copy,
{
    /// Start node to traverse from.
    pub fn start(node: DT<'a, T, U>) -> Traverse<'a, T, U> {
        Traverse {
            current: Some(node.0),
        }
    }

    /// Traverse to next node based on its decision.
    ///
    /// If none of the operations is met, return `None`.
    pub fn traverse(&mut self, decision: U, partial_op: PartialOp) -> Option<DT<'a, T, U>> {
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
