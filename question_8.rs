#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let node = node.borrow();
            let left_depth = max_depth(node.left.clone());
            let right_depth = max_depth(node.right.clone());
            1 + left_depth.max(right_depth)
        },
        None => 0,
    }
}

fn main() {
    let mut root = TreeNode::new(3);
    let mut node9 = TreeNode::new(9);
    let mut node20 = TreeNode::new(20);
    let mut node15 = TreeNode::new(15);
    let mut node7 = TreeNode::new(7);

    node20.left = Some(Rc::new(RefCell::new(node15)));
    node20.right = Some(Rc::new(RefCell::new(node7)));

    root.left = Some(Rc::new(RefCell::new(node9)));
    root.right = Some(Rc::new(RefCell::new(node20)));

    let depth = max_depth(Some(Rc::new(RefCell::new(root))));
    println!("Maximum depth of the tree: {}", depth); // Output: 3
}
