struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => 1 + std::cmp::max(max_depth(&node.left), max_depth(&node.right)),
    }
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 2,
            left:Some(Box::new(TreeNode {
            val: 4,
            left: None,
            right: None,
           })),
           
           
            right: Some(Box::new(TreeNode {
            val: 5,
            left: None,
            right: None,
          })),
          
        })),
        
        right: Some(Box::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })),
        
    }));
    let max_depth = max_depth(&root);
    println!("The maximum depth of the tree is: {}", max_depth);
}