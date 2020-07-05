pub enum BinaryTree<BranchValue, LeafValue> {
    Branch {
        value: BranchValue,
        left: Box<Self>,
        right: Box<Self>,
    },
    Leaf {
        value: LeafValue,
    },
    None,
}
