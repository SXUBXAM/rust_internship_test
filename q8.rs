use std::rc::Rc;
use std::cell::RefCell;

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(node.borrow().left.clone());
            let right_depth = max_depth(node.borrow().right.clone());
            1 + left_depth.max(right_depth)
        }
    }
}
