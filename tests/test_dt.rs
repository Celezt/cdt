#[cfg(test)]
mod tests {
    use cdt::DT;

    #[test]
    fn test_dt() {
        let mut root = DT::new("data1", true);
        let node = DT::new("data2", false);
        println!("{:#?}", root);
        println!("{:#?}", node);
        println!("{}", root.len());

        root.append(node.clone());
        println!("{}", node.last_parent().unwrap());

        println!("{}", root.len());

        root.append(DT::new("data3", true))
            .last_child()
            .unwrap()
            .append(DT::new("child1", true))
            .append(DT::new("child2", false))
            .last_child()
            .unwrap()
            .append(DT::new("child3", true))
            .append(DT::new("child4", false))
            .last_parent()
            .unwrap()
            .last_parent()
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
}
