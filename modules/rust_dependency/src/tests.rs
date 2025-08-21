#[cfg(test)]
mod nmide_test {
    use crate::get_graph;
    use std::path::Path;

    #[test]
    fn graphiphy_nmide_dir() {
        let path = Path::new("../../").canonicalize().unwrap();
        let graph = get_graph(&path);
        assert!(graph.list().is_some_and(|l| !l.is_empty()));
    }
}
