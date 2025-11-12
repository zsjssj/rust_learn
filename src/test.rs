use crate::learn_more::learn_algorithm::learn_01::Tree;
use crate::learn_more::learn_algorithm::learn_01::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

pub fn run() {
    let root_node: Tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode { val: 2, left: None, right: None }))),
        right: Some(Rc::new(RefCell::new(TreeNode { val: 3, left: None, right: None }))),
    })));
    let start = Instant::now();
    let v1 = min_depth(root_node.clone());
    let duration = start.elapsed();
    println!("计算耗时: {:?}", duration);
    println!("二叉树的最小深度为: {}", v1);
}

fn min_depth(root: Tree) -> i32 {
    match root {
        Some(node) => {
            let node_borrow = node.borrow();

            let left_depth = min_depth(node_borrow.left.clone());
            let right_depth = min_depth(node_borrow.right.clone());

            if left_depth == 0 || right_depth == 0 {
                return left_depth.max(right_depth) + 1;
            }
            left_depth.min(right_depth) + 1
        }
        None => 0,
    }
}
