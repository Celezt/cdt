#[cfg(test)]
mod tests {
    use cdt::{PartialOp, Traverse, DT};

    #[test]
    fn test_dt() {
        let mut tree = DT::init();

        tree.append("1", "data1", 1)
            .append("2", "data2", 2)
            .append("3", "data3", 3)
            .append("4", "data4", 4)
            .append("5", "data5", 5)
            .append("6", "data6", 6);

        tree.find("1")
            .unwrap()
            .append("7", "data7", 7)
            .append("8", "data8", 8);

        let mut travel = Traverse::start(tree);
        println!("{:?}", travel.traverse(1, PartialOp::Equal));
        println!("{:?}", travel.traverse(8, PartialOp::Equal));
    }

    #[test]
    fn test_len() {
        let mut tree = DT::init();
        tree.append("1", "data1", 1)
            .latest_child()
            .unwrap()
            .append("7", "child1", 7)
            .append("8", "child1", 7)
            .latest_parent()
            .unwrap()
            .append("2", "data2", 2)
            .append("3", "data3", 3)
            .append("4", "data4", 4)
            .append("5", "data5", 5)
            .append("6", "data6", 6);
        assert!(tree.len() == 6);
        assert!(tree.tree_len() == 9);
    }

    #[test]
    #[should_panic(
        expected = "Not allowed to append a node with the same id as one that already exist."
    )]
    fn test_same_id() {
        let mut tree = DT::init();
        tree.append("1", "data1", 1).append("1", "data2", 2);
    }

    #[test]
    fn test_fn_pointers() {
        #[derive(Copy, Clone, Debug)]
        struct Package<T>(T);
        impl<T> std::ops::Deref for Package<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        fn a() {
            println!("a");
        }

        fn b() {
            println!("b");
        }
        fn c() {
            println!("c");
        }
        fn d() {
            println!("d");
        }

        let mut tree: DT<Package<fn()>, i32> = DT::init();
        tree.append("1", Package(a), 1)
            .append("2", Package(b), 2)
            .latest_child()
            .unwrap()
            .append("3", Package(c), 3)
            .append("4", Package(d), 4);
        let mut travel = Traverse::start(tree);
        travel
            .traverse(2, PartialOp::Equal)
            .unwrap()
            .content()
            .unwrap()();
        travel
            .traverse(3, PartialOp::Equal)
            .unwrap()
            .content()
            .unwrap()();
    }
}
