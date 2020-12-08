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

        tree.find("4")
            .unwrap()
            .append("7", "data7", 7)
            .append("8", "data8", 8);

        tree.find("7")
            .unwrap()
            .append("9", "data9", 9)
            .append("10", "data10", 10);

        let mut travel = Traverse::start(tree);
        println!("{:?}", travel.traverse(4, PartialOp::Equal));
        println!("{:?}", travel.traverse(1, PartialOp::Less));
        println!("{:?}", travel.traverse(1, PartialOp::Less));
    }

    #[test]
    fn test_empty_parent() {
        let mut tree = DT::init();

        tree.append("id", "data", 1);

        let mut travel = Traverse::start(tree);
        assert!(travel.traverse(2, PartialOp::Greater).is_some());
        assert!(travel.traverse(0, PartialOp::Equal).is_none());
    }

    #[test]
    fn test_partial_op() {
        let mut tree = DT::init();

        tree.append("1", "data1", "a")
            .append("2", "data2", "b")
            .append("3", "data3", "c");

        tree.find("2")
            .unwrap()
            .append("4", "data4", "d")
            .append("5", "data5", "e");

        tree.find("4")
            .unwrap()
            .append("6", "data6", "f")
            .append("7", "data7", "g");

        let mut travel = Traverse::start(tree);
        println!("{:?}", travel.traverse("b", PartialOp::Equal));
        println!("{:?}", travel.traverse("b", PartialOp::Less));
        println!("{:?}", travel.traverse("g", PartialOp::Equal));
        /* assert!(
            travel
                .traverse("b", PartialOp::Median)
                .unwrap()
                .decision()
                .unwrap()
                == "b"
        );
        assert!(
            travel
                .traverse("c", PartialOp::Less)
                .unwrap()
                .decision()
                .unwrap()
                == "d"
        );
        assert!(
            travel
                .traverse("g", PartialOp::Max)
                .unwrap()
                .decision()
                .unwrap()
                == "g"
        ); */
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
