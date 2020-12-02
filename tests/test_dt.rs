#[cfg(test)]
mod tests {
    use cdt::{PartialOp, Traverse, DT};

    #[test]
    fn test_dt() {
        let mut root = DT::init("data1");

        root.append(DT::new("data3", true))
            .latest_child()
            .unwrap()
            .append(DT::new("child1", true))
            .append(DT::new("child2", false))
            .latest_child()
            .unwrap()
            .append(DT::new("child3", true))
            .append(DT::new("child4", false))
            .latest_parent()
            .unwrap()
            .latest_parent()
            .unwrap()
            .append(DT::new("data4", false))
            .append(DT::new("data5", false));
    }

    #[test]
    #[should_panic(expected = "Not legal to append to itself")]
    fn test_overwrite_itself() {
        // Test appending itself
        let mut root = DT::new("root", true);
        root.append(root.clone());
    }

    #[test]
    fn test_move() {
        let mut root = DT::init("data1");
        root.append(DT::new("data2", false))
            .latest_child()
            .unwrap()
            .append(DT::new("data3", true))
            .latest_child()
            .unwrap()
            .append(DT::new("data4", false))
            .latest_child()
            .unwrap()
            .append(DT::new("data5", true))
            .latest_child()
            .unwrap()
            .append(DT::new("data6", false))
            .latest_child()
            .unwrap()
            .append(DT::new("data7", true))
            .append(DT::new("data8", false));

        println!("{:#?}", root.first());
        assert!(root.first().is_some());
        println!("{:#?}", root.last());
        assert!(root.last().is_some());

        println!("{:#?}", root.forward_first(6));

        println!("{:#?}", root.forward_first(6));
        assert!(root.forward_first(6).is_some());
        assert!(root.forward_first(7).is_none());

        println!("{:#?}", root.forward_last(6));
        assert!(root.forward_last(6).is_some());
    }

    #[test]
    fn test_iterate_1() {
        let mut root = DT::init("0_0");
        root.append(DT::new("1", 2))
            .latest_child()
            .unwrap()
            .append(DT::new("1_1", 3))
            .append(DT::new("1_2", 4))
            .latest_parent()
            .unwrap()
            .append(DT::new("2", 5))
            .latest_child()
            .unwrap()
            .append(DT::new("2_1", 8))
            .append(DT::new("2_2", 4))
            .latest_child()
            .unwrap()
            .append(DT::new("2_2_1", 2))
            .append(DT::new("2_2_2", 1))
            .append(DT::new("2_2_3", 4));

        let mut travel = Traverse::start(root);

        println!("{:?}", travel.traverse(3, PartialOp::Less));
        println!("{:?}", travel.traverse(5, PartialOp::Greater));
        println!("{:?}", travel.traverse(4, PartialOp::Equal));
    }

    #[test]
    fn test_iterate_2() {
        #[derive(Debug)]
        struct Data<'a> {
            text: &'a str,
        }
        let mut root = DT::init(Data { text: "init" });
        root.append(DT::new(Data { text: "hey" }, 'a'))
            .latest_child()
            .unwrap()
            .append(DT::new(Data { text: "what" }, 'b'))
            .append(DT::new(Data { text: "is" }, 'c'))
            .latest_parent()
            .unwrap()
            .append(DT::new(Data { text: "up" }, 'd'))
            .latest_child()
            .unwrap()
            .append(DT::new(Data { text: "are" }, 'e'))
            .append(DT::new(Data { text: "you" }, 'f'))
            .latest_child()
            .unwrap()
            .append(DT::new(Data { text: "alright" }, 'g'))
            .append(DT::new(Data { text: "bro" }, 'h'))
            .append(DT::new(Data { text: "?" }, 'i'));

        let mut travel = Traverse::start(root);

        println!("{:?}", travel.traverse('c', PartialOp::Less));
        println!("{:?}", travel.traverse('f', PartialOp::Equal));
        println!("{:?}", travel.traverse('h', PartialOp::Greater));
    }
}
