#[cfg(test)]
mod tests {
    use cdt::{Op, Traverse, DT};

    #[test]
    fn test_dt() {
        let mut tree = DT::init();

        tree.append("1", "data1", 1, Op::Equal)
            .append("2", "data2", 2, Op::Equal)
            .append("3", "data3", 3, Op::Equal)
            .append("4", "data4", 4, Op::Equal)
            .append("5", "data5", 5, Op::Equal)
            .append("6", "data6", 6, Op::Equal);

        tree.find("4")
            .unwrap()
            .append("7", "data7", 7, Op::Less)
            .append("8", "data8", 8, Op::Less);

        tree.find("7")
            .unwrap()
            .append("9", "data9", 9, Op::Less)
            .append("10", "data10", 10, Op::Less);

        let mut travel = Traverse::start(tree);
        println!("{:?}", travel.traverse(&4));
        println!("{:?}", travel.traverse(&1));
        println!("{:?}", travel.traverse(&1));
    }

    #[test]
    fn test_empty_parent() {
        let mut tree = DT::init();

        tree.append("id", "data", 1, Op::Greater);

        let mut travel = Traverse::start(tree);
        assert!(travel.traverse(&2).is_some());
        assert!(travel.traverse(&0).is_none());
    }

    #[test]
    fn test_partial_op() {
        let mut tree = DT::init();

        tree.append("1", "data1", "a", Op::Equal)
            .append("2", "data2", "b", Op::Equal)
            .append("3", "data3", "c", Op::Equal);

        tree.find("2")
            .unwrap()
            .append("4", "data4", "d", Op::Less)
            .append("5", "data5", "e", Op::Less);

        tree.find("4")
            .unwrap()
            .append("6", "data6", "f", Op::Greater)
            .append("7", "data7", "g", Op::Greater);

        let mut travel = Traverse::start(tree);
        assert!(travel.traverse(&"b").unwrap().decision().unwrap() == "b");
        assert!(travel.traverse(&"c").unwrap().decision().unwrap() == "d");
        assert!(travel.traverse(&"h").unwrap().decision().unwrap() == "f");
    }

    #[test]
    fn test_len() {
        let mut tree = DT::init();
        tree.append("1", "data1", 1, Op::Equal)
            .latest_child()
            .unwrap()
            .append("7", "child1", 7, Op::Equal)
            .append("8", "child1", 7, Op::Equal)
            .latest_parent()
            .unwrap()
            .append("2", "data2", 2, Op::Equal)
            .append("3", "data3", 3, Op::Equal)
            .append("4", "data4", 4, Op::Equal)
            .append("5", "data5", 5, Op::Equal)
            .append("6", "data6", 6, Op::Equal);
        assert!(tree.len() == 6);
        assert!(tree.tree_len() == 9);
    }

    #[test]
    #[should_panic(
        expected = "Not allowed to append a node with the same id as one that already exist."
    )]
    fn test_same_id() {
        let mut tree = DT::init();
        tree.append("1", "data1", 1, Op::Equal)
            .append("1", "data2", 2, Op::Equal);
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
        tree.append("1", Package(a), 1, Op::Equal)
            .append("2", Package(b), 2, Op::Equal)
            .latest_child()
            .unwrap()
            .append("3", Package(c), 3, Op::Equal)
            .append("4", Package(d), 4, Op::Equal);
        let mut travel = Traverse::start(tree);
        travel.traverse(&2).unwrap().content().unwrap()();
        travel.traverse(&3).unwrap().content().unwrap()();
    }
}
