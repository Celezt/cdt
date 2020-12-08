 # CDT

 `cdt`, also called Celezt's Decision Tree is a
 library that implements DT to rust.

 It is inspired by these repositories:
 https://github.com/SimonSapin/rust-forest
 https://github.com/RazrFalcon/rctree/blob/master/src/lib.rs

 Decision Tree

 Generic implementation that takes in a unique id `&str` that is implemented as
 a hash map. It decides what route to take base on the decision that has `PartialEq` +
 `PartialOrd` + `Copy` derived. By using the `Traverse` struct, it is possible to move
 along the tree.

 # Examples

 ```
 use cdt::{DT, Traverse, Op};

 // Initialize a new decision tree by creating a root that
 // has by default the id "root"
 let mut tree = DT::init();

 /// Append new children to the root
 /// .append(unique id, data, decision, operator)
 tree.append("first", "banana", true, Op::Equal)
     .append("second", "apple", false, Op::Equal)
     .append("third", "orange", false, Op::Equal);

 /// Get node by its id
 tree.find("second").unwrap()
     .append("fourth", "red apple", true, Op::Equal)
     .append("fifth", "green apple", false, Op::Equal);

 let mut travel = Traverse::start(tree);

 // apple because it is the first one that are false
 // The decision goes from left to right (top to bottom)
 assert!(travel.traverse(&false).unwrap().decision().unwrap() == false);

 // The first one of apple's children that are true
 assert!(travel.traverse(&true).unwrap().decision().unwrap() == true);
 ```