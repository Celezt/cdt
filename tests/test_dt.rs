#[cfg(test)]
mod tests {
    use cdt::DT;

    #[test]
    fn test_dt() {
        let mut root = DT::new("data1", true);
        let mut node = DT::new("data2", false);
        println!("{:#?}", root);
        println!("{:#?}", node);
        println!("{}", root.child_len());
        root.append(node);
        println!("{}", root.child_len());
        root.append(DT::new("data3", true));
        println!("{}", root.child_len());
    }
}
