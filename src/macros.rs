/// Return value if `Some`, else return `None`.
#[macro_export]
macro_rules! try_opt {
    ($expr: expr) => {
        match $expr {
            Some(value) => value,
            None => return None,
        }
    };
}

/* #[macro_export]
macro_rules! decision_tree {
    ($($data:tt : $decision:tt $(= $child:expr)?),+ $(,)?) => {{
        let mut root = DT::init();
        $(root.append(DT::new($data, $decision));)+
        root
    }};
} */
