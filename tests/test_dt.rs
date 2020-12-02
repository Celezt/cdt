#[cfg(test)]
mod tests {
    use cdt::Iterate;
    use cdt::DT;

    #[test]
    fn test_dt() {
        let mut root = DT::new("data1", true);
        let node = DT::new("data2", false);
        println!("{:#?}", root);
        println!("{:#?}", node);
        println!("{}", root.len());

        root.append(node.clone());
        println!("{}", node.latest_parent().unwrap());

        println!("{}", root.len());

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

        println!("{}", root.len());

        println!("{:#?}", root.child(1).unwrap().child(0).unwrap());
        println!("{:#?}", root.child(1).unwrap().child(0).unwrap().root());
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
        let mut root = DT::new("data1", true);
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
    fn test_iterate() {
        let mut root = DT::new("data1", true);
        root.append(DT::new("1_0", false))
            .latest_child()
            .unwrap()
            .append(DT::new("1_1", true))
            .append(DT::new("1_2", true))
            .latest_parent()
            .unwrap()
            .append(DT::new("2_0", true))
            .latest_child()
            .unwrap()
            .append(DT::new("2_1", true))
            .append(DT::new("2_2", true));

        let mut iter = Iterate::start(root);
        println!("{:#?}", iter.traverse(false));
    }
}
