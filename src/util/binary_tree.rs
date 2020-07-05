pub enum BinaryTree<Value, LeafValue>
{
    Branch {
        value: Value,
        left: Box<Self>,
        right: Box<Self>,
    },
    Leaf {
        value: LeafValue,
    },
    None,
}

impl<Value, LeafValue> BinaryTree<Value, LeafValue>
{
    pub fn count_leaves(&self) -> usize {
        match self {
            Self::Branch {
                value: _,
                left,
                right,
            } => right.count_leaves() + left.count_leaves(),
            Self::Leaf { value: _ } => 1,
            Self::None => 0,
        }
    }
}
