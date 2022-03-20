#[cfg(test)]
mod tests {
    use crate::ast;
    use std::assert_matches::assert_matches;

    use crate::tree::NodeInfo;
    use crate::tree::*;

    #[test]
    fn test_finds_nothing_in_empty_tree() {
        let tree = ast::empty();
        let mut path = ast::Path::root();
        assert_eq!(None, path.follow(&tree));
    }

    #[test]
    fn test_finds_something_at_root_of_leaf_tree() {
        let tree = ast::leaf(NodeInfo::Constant(42));
        let mut path = ast::Path::root();
        assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Constant(42));
    }

    #[test]
    fn test_finds_something_inside_tree() {
        let child = ast::leaf(NodeInfo::Constant(42));
        let sibling = ast::leaf(NodeInfo::Constant(10));
        let node = ast::node(NodeInfo::Term(TermOp::Times), child, sibling);

        let mut root = ast::Path::root();
        let path = root.child();
        assert_matches!(path.follow(&node).unwrap(), NodeInfo::Constant(42));

        let mut root = ast::Path::root();
        let path = root.sibling();
        assert_matches!(path.follow(&node).unwrap(), NodeInfo::Constant(10));
    }
}
