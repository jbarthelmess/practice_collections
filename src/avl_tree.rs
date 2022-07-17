/* Tree Node which holds the balance of it's subtrees

Negative values will indicate that the left tree is deeper than the right tree
Positive values will indicate that the right tree is deeper than the left tree
0 will indicate equal heights. 

Adding and subtracting will work to maintain the integrity of the balance value 
of each node in the tree, by updating as we go, and greedily rotating 
overbalanced trees (trees whose balance becomes -2 or 2)*/
struct Node<T> {
    balance: i8,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    val: T
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            balance: 0,
            left: None,
            right: None,
            val
        }
    }
}

fn rotate_left<T>(mut root: Box<Node<T>>) -> Box<Node<T>> {
    if let Some(mut child) = root.right {
        if child.balance < 0 {
            child = rotate_right(child);
        }
        root.right = child.left;
        (root.balance, child.balance) = match (root.balance, child.balance) {
            (2, 1) => (0, 0),
            (2, 0) => (1, -1),
            (1, 1) => (-1, -1),
            (1, 0) => (0, -1),
            _ => unreachable!("These are all the valid cases for left rotation")
        };
        child.left = Some(root);
        child
    } else {
        unreachable!("Left rotation cannot be called on function with no right child");
    }
}

fn rotate_right<T>(mut root: Box<Node<T>>) -> Box<Node<T>> {
    if let Some(mut child) = root.left {
        if child.balance > 0 {
            child = rotate_left(child);
        }
        root.left = child.right;
        (root.balance, child.balance) = match (root.balance, child.balance) {
            (-2, -1) => (0, 0),
            (-2, 0) => (-1, 1),
            (-1, -1) => (1, 1),
            (-1, 0) => (0, 1),
            _ => unreachable!("These are all the valid cases for right rotation")
        };
        child.right = Some(root);
        child
    } else {
        unreachable!("Right rotation cannot be called on function with no left child");
    }
}

fn add_helper<T: Ord>(root: Option<Box<Node<T>>>, val: T) -> (Option<Box<Node<T>>>, i8) {
    if let Some(mut node) = root {
        let mut height_change = 0;
        if node.val > val {
            (node.left, height_change) = add_helper(node.left, val);
            node.balance -= height_change;
            if node.balance == -2 {
                node = rotate_right(node);
                height_change = 0;
            } else if height_change != 0 {
                height_change = node.balance*(-1);
            }
        } else if node.val < val {
            (node.right, height_change) = add_helper(node.right, val);
            node.balance += height_change;
            if node.balance == 2 {
                node = rotate_left(node);
                height_change = 0;
            } else if height_change != 0 {
                height_change = node.balance;
            }
        }
        (Some(node), height_change)
    } else {
        (Some(Box::new(Node::new(val))), 1)
    }
}

fn remove_height_change_update<T>(mut root: Box<Node<T>>) -> (Box<Node<T>>, i8) {
    let ret_val = match root.balance {
        2 => {
            let release = if let Some(ref r) = root.right {
                r.balance.abs()
            } else {
                unreachable!("Right heavy tree must have right child")
            };
            root = rotate_left(root);
            release
        },
        -2 => {
            let release = if let Some(ref l) = root.left {
                l.balance.abs()
            } else {
                unreachable!("Left heavy tree must have left child")
            };
            root = rotate_right(root);
            release
        },
        1 | -1 => 0,
        0 => 1,
        _ => unreachable!("Node balance can only be -2, -1, 0, 1, 2, was {}", root.balance)
    };
    (root, ret_val)
}

fn remove_least<T>(mut root: Box<Node<T>>) -> (Option<Box<Node<T>>>, T, i8) {
    if let Some(node) = root.left {
        let new_val;
        let mut height_change;
        (root.left, new_val, height_change) = remove_least(node);
        root.balance += height_change;
        if height_change != 0 {
            (root, height_change) = remove_height_change_update(root);
        }
        (Some(root), new_val, height_change)
    } else {// we found the least element
        (root.right, root.val, 1)
    }
}

fn remove_greatest<T>(mut root: Box<Node<T>>) -> (Option<Box<Node<T>>>, T, i8) {
    if let Some(node) = root.right {
        let new_val;
        let mut height_change;
        (root.right, new_val, height_change) = remove_greatest(node);
        root.balance -= height_change;
        if height_change != 0 {
            (root, height_change) = remove_height_change_update(root);
        }
        (Some(root), new_val, height_change)
    } else { // we found the greatest element
        (root.left, root.val, 1)
    }
}

fn remove_helper<T: Ord>(root: Option<Box<Node<T>>>, val: T) -> (Option<Box<Node<T>>>, i8) {
    if let Some(mut node) = root {
        if node.val == val {
            match (node.left, node.right) {
                (None, None) => (None, 1),
                (Some(node), None) | (None, Some(node)) => (Some(node), 1),
                (Some(left), Some(right)) => {
                    if node.balance > 0 { // tree is right heavy
                        let (new_right, new_val, height_change) = remove_least(right);
                        node.right = new_right;
                        node.left = Some(left);
                        node.val = new_val;
                        node.balance -= height_change;
                        (Some(node), 0)
                    } else { // tree is balanced, or left heavy
                        let (new_left, new_val, height_change) = remove_greatest(left);
                        node.left = new_left;
                        node.right = Some(right);
                        node.val = new_val;
                        node.balance += height_change;
                        (Some(node), 0)
                    }
                }
            }
        } else if node.val > val {
            let mut height_change;
            (node.left, height_change) = remove_helper(node.left, val);
            node.balance += height_change;
            if height_change != 0 {
                (node, height_change) = remove_height_change_update(node);
            }
            (Some(node), height_change)
        } else {
            let mut height_change;
            (node.right, height_change) = remove_helper(node.right, val);
            node.balance -= height_change;
            if height_change != 0 {
                (node, height_change) = remove_height_change_update(node);
            }
            (Some(node), height_change)
        }
    } else {
        (None, 0)
    }
}

fn find<T: Ord>(root: &Option<Box<Node<T>>>, val: T) -> bool {
    if let Some(node) = root {
        if node.val == val {
            true
        } else if node.val < val {
            find(&node.right, val)
        } else {
            find(&node.left, val)
        }
    } else {
        false
    }
}

pub struct BinaryTree<T> {
    head: Option<Box<Node<T>>>,
    depth: usize
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            depth: 0
        }
    }

    pub fn add(&mut self, val: T) {
        let (new_head, added_depth) = add_helper(self.head.take(), val);
        self.head = new_head;
        self.depth += added_depth as usize;
    }

    pub fn remove(&mut self, val: T) {
        let (new_head, removed_depth) = remove_helper(self.head.take(), val);
        self.head = new_head;
        self.depth -= removed_depth as usize;
    }

    pub fn contains(&self, val: T) -> bool {
        find(&self.head, val)
    }

    pub fn iter(&self) -> TreeIterator<T> {
        TreeIterator::new(self)
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}

pub struct TreeIterator<'a, T:'a> {
    stack: Vec<&'a Box<Node<T>>>
}

impl<'a, T> TreeIterator<'a, T> {
    fn new(tree: &'a BinaryTree<T>) -> Self {
        let mut stack = Vec::new();
        let mut begin = tree.head.as_ref();
        while let Some(val) = begin {
            stack.push(val);
            begin = val.left.as_ref();
        }
        Self { stack }
    }
}

impl<'a, T: 'a+Ord> Iterator for TreeIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let release = self.stack.pop();
        if let Some(node) = release {
            let mut begin = node.right.as_ref();
            while let Some(node) = begin {
                self.stack.push(node);
                begin = node.left.as_ref();
            }
            Some(&node.val)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Display;

    fn print_node_tree<T: Display>(tree: Option<&Box<Node<T>>>, depth: usize) {
        if let Some(node) = tree {
            print_node_tree(node.left.as_ref(), depth+1);
            println!("{}{}:{}", " ".repeat(2*depth), node.val, node.balance);
            print_node_tree(node.right.as_ref(), depth+1);
        }
    }
    
    // Test Node Addition first
    mod add_tests {
        use super::*;

        #[test]
        fn node_add_right() {
            let root = Some(Box::new(Node::new(1)));
            let (root, height_change) = add_helper(root, 2);
            assert_eq!(height_change, 1);
            if let Some(node) = root {
                assert!(node.left.is_none());
                assert!(!node.right.is_none());
                assert_eq!(node.balance, 1);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
        }

        #[test]
        fn node_add_left() {
            let root = Some(Box::new(Node::new(1)));
            let (root, height_change) = add_helper(root, 0);
            assert_eq!(height_change, 1);
            if let Some(node) = root {
                assert!(!node.left.is_none());
                assert!(node.right.is_none());
                assert_eq!(node.balance, -1);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
        }

        #[test]
        fn node_add_duplicate() {
            let root = Some(Box::new(Node::new(1)));
            let (root, height_change) = add_helper(root, 1);
            assert_eq!(height_change, 0);
            if let Some(node) = root {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
                assert_eq!(node.balance, 0);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
        }

        #[test]
        fn simple_left_rotation() {
            // (-2, -1) case in the rotate_left function
            let root = Some(Box::new(Node::new(1)));
            let (root, _) = add_helper(root, 2);
            /* before
                1
                 \
                  2
            */
            let (root, height_change) = add_helper(root, 3);
            /* after
                2
               / \
              1   3
            */
            assert_eq!(height_change, 0);
            if let Some(node) = root {
                assert!(!node.left.is_none());
                assert!(!node.right.is_none());
                assert_eq!(node.balance, 0);
                assert_eq!(node.val, 2);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
        }

        #[test]
        fn simple_right_rotation() {
            // (2, 1) case in the rotate_right function
            let root = Some(Box::new(Node::new(3)));
            let (root, _) = add_helper(root, 2);
            /* before
                3
               / 
              2   
            */
            let (root, height_change) = add_helper(root, 1);
            /* after
                2
               / \
              1   3
            */
            assert_eq!(height_change, 0);
            if let Some(node) = root {
                assert!(!node.left.is_none());
                assert!(!node.right.is_none());
                assert_eq!(node.balance, 0);
                assert_eq!(node.val, 2);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
        }

        #[test]
        fn right_rotation_left_heavy_left_child() {
            // (-2, 1) case again
            let root = Some(Box::new(Node::new(5)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 7);
            println!("After adding left and right\n");
            print_node_tree(root.as_ref(), 0);

            let (root, _) = add_helper(root, 1);
            println!("After adding another left\n");
            print_node_tree(root.as_ref(), 0);

            let (root, _) = add_helper(root, 3);
            println!("After adding another left\n");
            print_node_tree(root.as_ref(), 0);

            if let Some(node) = &root {
                //assert_eq!(node.val, 5);
                assert_eq!(node.balance, -1);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            // up until now, no rotations have occurred
            /* before
                5
               / \
              2   7
             / \
            1   3
            */
            let (root, height_change) = add_helper(root, 0);
            /* transformation
                  5             2
                 / \    ->     / \
                2   7         1   5
               / \           /   / \
              1   3         0   3   7
             /  
            0    
            because the top is left heavy, it needs to make it's left child the new root. 
            it loses its left child as a result, but the left child must make the old root, it's new right child,
            if it already had a right child that node will become the old root's new left child.
            */
            println!("After rotations...");
            print_node_tree(root.as_ref(), 0);
            if let Some(node) = root {
                assert_ne!(node.val, 5);
                assert_eq!(node.val, 2);
                assert_eq!(height_change, 0);
                assert_eq!(node.balance, 0);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
        }

        #[test]
        fn right_rotation_right_heavy_left_child() {
            // (-2, 1) case should transform to (-2, -1) case before rotating root
            let root = Some(Box::new(Node::new(5)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 7);
            let (root, _) = add_helper(root, 1);
            let (root, _) = add_helper(root, 3);
            if let Some(node) = &root {
                assert_eq!(node.val, 5);
                assert_eq!(node.balance, -1);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            // up until now, no rotations have occurred
            /* before
                5
               / \
              2   7
             / \
            1   3
            */
            let (root, height_change) = add_helper(root, 4);
            /* transformations
                5             5              3
               / \    ->     / \     ->     / \
              2   7         3   7          2   5
             / \           / \            /   / \
            1   3         2   4          1   4   7
                 \       /
                  4     1
            first the tree rotates the left subtree to be left heavy instead of right heavy
            it then rotates the top to balance the tree
            */
            println!("After rotations...");
            print_node_tree(root.as_ref(), 0);
            if let Some(node) = root {
                assert_ne!(node.val, 5);
                assert_eq!(node.val, 3);
                assert_eq!(height_change, 0);
                assert_eq!(node.balance, 0);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
        }

        #[test]
        fn right_rotation_left_heavy_left_child_no_right_child() {
            let root = Some(Box::new(Node::new(5)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 7);
            let (root, _) = add_helper(root, 1);
            if let Some(node) = &root {
                assert_eq!(node.val, 5);
                assert_eq!(node.balance, -1);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            // up until now, no rotations have occurred
            /* before
                5
               / \
              2   7
             /
            1  
            */
            let (root, height_change) = add_helper(root, 0);
            /* transformations greedily so will rotate bottom of tree first, instead of top
                  5             5
                 / \    ->     / \
                2   7         1   7
               /             / \    
              1             0   2    
             /  
            0    
            */
            println!("After rotations...");
            print_node_tree(root.as_ref(), 0);
            if let Some(node) = root {
                assert_eq!(node.val, 5);
                assert_eq!(height_change, 0);
                assert_eq!(node.balance, -1);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
        }

        #[test]
        fn right_rotation_right_heavy_left_child_no_left_child() {
            let root = Some(Box::new(Node::new(5)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 7);
            let (root, _) = add_helper(root, 3);
            if let Some(node) = &root {
                assert_eq!(node.val, 5);
                assert_eq!(node.balance, -1);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            // up until now, no rotations have occurred
            /* before
                5
               / \
              2   7
               \
                3
            */
            let (root, height_change) = add_helper(root, 4);
            /* transformations
                5             5 
               / \    ->     / \     
              2   7         3   7          
               \           / \            
                3         2   4          
                 \        
                  4      
            */
            println!("After rotations...");
            print_node_tree(root.as_ref(), 0);
            if let Some(node) = root {
                assert_eq!(node.val, 5);
                assert_eq!(height_change, 0);
                assert_eq!(node.balance, -1);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
        }

        #[test]
        fn left_rotation_left_heavy_right_child() {
            let root = Some(Box::new(Node::new(4)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 7);
            let (root, _) = add_helper(root, 6);
            let (root, _) = add_helper(root, 8);
            if let Some(node) = &root {
                assert_eq!(node.val, 4);
                assert_eq!(node.balance, 1);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            // up until now, no rotations have occurred
            /* before
                4
               / \
              2   7
                 / \
                6   8
            */
            let (root, height_change) = add_helper(root, 5);
            /* transformation
                   4                 4                 6
                  / \      ->       / \       ->      / \
                 2   7             2   6             4   7
                    / \               / \           / \   \
                   6   8             5   7         2   5   8
                  /                       \
                 5                         8
            because the top is right heavy, it needs to make it's right child the new root. 
            it loses its right child as a result, but the right child must make the old root, it's new left child,
            if it already had a left child that node will become the old root's new left child.
            */

            println!("After rotations...");
            print_node_tree(root.as_ref(), 0);
            if let Some(node) = root {
                assert_ne!(node.val, 4);
                assert_eq!(node.val, 6);
                assert_eq!(height_change, 0);
                assert_eq!(node.balance, 0);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
        }

        #[test]
        fn left_rotation_right_heavy_right_child() {
            let root = Some(Box::new(Node::new(4)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 6);
            let (root, _) = add_helper(root, 5);
            let (root, _) = add_helper(root, 7);
            if let Some(node) = &root {
                assert_eq!(node.val, 4);
                assert_eq!(node.balance, 1);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            // up until now, no rotations have occurred
            /* before
                4
               / \
              2   6
                 / \
                5   7
            */
            let (root, height_change) = add_helper(root, 8);
            /* transformations
                4             6
               / \    ->     / \
              2   6         4   7
                 / \       / \   \
                5   7     2   5   8
                     \  
                      8 
            */
            println!("After rotations...");
            print_node_tree(root.as_ref(), 0);
            if let Some(node) = root {
                assert_ne!(node.val, 4);
                assert_eq!(node.val, 6);
                assert_eq!(height_change, 0);
                assert_eq!(node.balance, 0);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
        }

        #[test]
        fn left_rotation_left_heavy_right_child_no_right_child() {
            let root = Some(Box::new(Node::new(4)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 7);
            let (root, _) = add_helper(root, 6);
            if let Some(node) = &root {
                assert_eq!(node.val, 4);
                assert_eq!(node.balance, 1);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            // up until now, no rotations have occurred
            /* before
                4
               / \
              2   7
                 /
                6  
            */
            let (root, height_change) = add_helper(root, 5);
            /* transformations greedily so will rotate bottom of tree first, instead of top
                   4             4
                  / \    ->     / \
                 2   7         2   6
                    /             / \    
                   6             5   7    
                  /  
                 5    
            */
            println!("After rotations...");
            print_node_tree(root.as_ref(), 0);
            if let Some(node) = root {
                assert_eq!(node.val, 4);
                assert_eq!(height_change, 0);
                assert_eq!(node.balance, 1);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
        }

        #[test]
        fn left_rotation_right_heavy_right_child_no_left_child() {
            let root = Some(Box::new(Node::new(5)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 7);
            let (root, _) = add_helper(root, 8);
            if let Some(node) = &root {
                assert_eq!(node.val, 5);
                assert_eq!(node.balance, 1);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            // up until now, no rotations have occurred
            /* before
                5
               / \
              2   7
                   \
                    8
            */
            let (root, height_change) = add_helper(root, 9);
            /* transformations
                5             5 
               / \    ->     / \     
              2   7         2   8          
                   \           / \            
                    8         7   9
                     \        
                      9      
            */
            println!("After rotations...");
            print_node_tree(root.as_ref(), 0);
            if let Some(node) = root {
                assert_eq!(node.val, 5);
                assert_eq!(height_change, 0);
                assert_eq!(node.balance, 1);
            } else {
                panic!("We didn't get a root node back from helper...");
            }
        }
    }

    mod remove_tests {
        use super::*;

        #[test]
        fn remove_root() {
            let root = Some(Box::new(Node::new(4)));
            let (root, height_change) = remove_helper(root, 4);
            assert!(root.is_none());
            assert_eq!(height_change, 1)
        }

        #[test]
        fn remove_left() {
            let root = Some(Box::new(Node::new(1)));
            let (root, _) = add_helper(root, 0);
            if let Some(node) = root.as_ref() {
                assert!(!node.left.is_none());
            } else {
                panic!("Add didn't return a root node, something went wrong...");
            }
            let (root, height_change) = remove_helper(root, 0);
            if let Some(node) = root {
                assert!(node.left.is_none());
                assert_eq!(node.val, 1);
                assert_eq!(height_change, 1);
            } else {
                panic!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_right() {
            let root = Some(Box::new(Node::new(1)));
            let (root, _) = add_helper(root, 2);
            if let Some(node) = root.as_ref() {
                assert!(!node.right.is_none());
            } else {
                panic!("Add didn't return a root node, something went wrong...");
            }
            let (root, height_change) = remove_helper(root, 2);
            if let Some(node) = root {
                assert!(node.right.is_none());
                assert_eq!(node.val, 1);
                assert_eq!(height_change, 1);
            } else {
                panic!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_root_with_left_child() {
            let root = Some(Box::new(Node::new(1)));
            let (root, _) = add_helper(root, 0);
            if let Some(node) = root.as_ref() {
                assert!(!node.left.is_none());
            } else {
                panic!("Add didn't return a root node, something went wrong...");
            }
            let (root, height_change) = remove_helper(root, 1);
            if let Some(node) = root {
                assert!(node.left.is_none());
                assert_ne!(node.val, 1);
                assert_eq!(node.val, 0);
                assert_eq!(height_change, 1);
            } else {
                panic!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_root_with_right_child() {
            let root = Some(Box::new(Node::new(1)));
            let (root, _) = add_helper(root, 2);
            if let Some(node) = root.as_ref() {
                assert!(!node.right.is_none());
            } else {
                panic!("Add didn't return a root node, something went wrong...");
            }
            let (root, height_change) = remove_helper(root, 1);
            if let Some(node) = root {
                assert!(node.right.is_none());
                assert_ne!(node.val, 1);
                assert_eq!(node.val, 2);
                assert_eq!(height_change, 1);
            } else {
                panic!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_root_with_two_children() {
            let root = Some(Box::new(Node::new(1)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 0);
            /*before
                    1
                   / \
                  0   2
             */
            if let Some(node) = root.as_ref() {
                assert!(!node.right.is_none());
                assert!(!node.left.is_none());
            } else {
                panic!("Add didn't return a root node, something went wrong...");
            }
            let (root, height_change) = remove_helper(root, 1);
            if let Some(node) = root {
                // biases towards pulling from the left tree in a balanced tree
                assert!(!node.right.is_none());
                assert!(node.left.is_none());
                assert_eq!(node.val, 0);
                assert_eq!(height_change, 0);
            } else {
                panic!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_root_with_two_children_left_heavy() {
            let root = Some(Box::new(Node::new(1)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, -1);
            let (root, _) = add_helper(root, 0);
            let (root, _) = add_helper(root, -2);
            /* before
                    1
                   / \
                 -1   2
                 / \
               -2   0
            */
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            let (root_wrapper, height_change) = remove_helper(root, 1);
            /* after
                    0
                   / \
                 -1   2
                 /
               -2  
            */
            println!("After rotations...");
            print_node_tree(root_wrapper.as_ref(), 0);
            if let Some(root) = root_wrapper {
                // pulls from the left when it is left heavy
                assert_eq!(root.val, 0);
                assert_eq!(height_change, 0);
            } else {
                panic!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_right_child_left_heavy() {
            let root = Some(Box::new(Node::new(1)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 0);
            let (root, _) = add_helper(root, -1);
            let (root, _) = add_helper(root, -2);
            /* before
                    1
                   / \
                 -1   2
                 / \
               -2   0
            */
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            let (root_wrapper, height_change) = remove_helper(root, 2);
            /* after
                    1            -1
                   /      ->     / \
                 -1            -2   1
                 / \               /
               -2   0             0
            */
            println!("After rotations...");
            print_node_tree(root_wrapper.as_ref(), 0);
            if let Some(root) = root_wrapper {
                // should have rotated right after removal
                assert_eq!(root.val, -1);
                assert_eq!(root.balance, 1);
                assert_eq!(height_change, 0);
                
            } else {
                panic!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_left_child_right_heavy() {
            let root = Some(Box::new(Node::new(0)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, -1);
            let (root, _) = add_helper(root, 3);
            let (root, _) = add_helper(root, 1);
            /* before
                    0
                   / \
                 -1   2
                     / \
                    1   3
            */
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            let (root_wrapper, height_change) = remove_helper(root, -1);
            /* after
                    0             2
                     \    ->     / \
                      2         0   3
                     / \         \      
                    1   3         1
            */
            println!("After rotations...");
            print_node_tree(root_wrapper.as_ref(), 0);
            if let Some(root) = root_wrapper {
                // should have rotated left after removal
                assert_eq!(root.val, 2);
                assert_eq!(root.balance, -1);
                assert_eq!(height_change, 0);
            } else {
                panic!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_right_child_left_heavy_no_neighbor() {
            let root = Some(Box::new(Node::new(1)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, -1);
            let (root, _) = add_helper(root, -2);
            /* before
                    1
                   / \
                 -1   2
                 /
               -2  
            */
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            let (root_wrapper, height_change) = remove_helper(root, 2);
            /* after
                    1            -1
                   /      ->     / \
                 -1            -2   1
                 /            
               -2                
            */
            println!("After rotations...");
            print_node_tree(root_wrapper.as_ref(), 0);
            if let Some(root) = root_wrapper {
                // should have rotated right after removal
                assert_eq!(root.val, -1);
                assert_eq!(root.balance, 0);
                assert_eq!(height_change, 1);
            } else {
                panic!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_left_child_right_heavy_no_neighbor() {
            let root = Some(Box::new(Node::new(0)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, -1);
            let (root, _) = add_helper(root, 3);
            /* before
                    0
                   / \
                 -1   2
                       \
                        3
            */
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            let (root_wrapper, height_change) = remove_helper(root, -1);
            /* after
                    0             2
                     \    ->     / \
                      2         0   3
                       \               
                        3         
            */
            println!("After rotations...");
            print_node_tree(root_wrapper.as_ref(), 0);
            if let Some(root) = root_wrapper {
                // should have rotated left after removal
                assert_eq!(root.val, 2);
                assert_eq!(root.balance, 0);
                assert_eq!(height_change, 1);
            } else {
                panic!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_right_child_left_heavy_opposite_neighbor() {
            let root = Some(Box::new(Node::new(1)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, -1);
            let (root, _) = add_helper(root, 0);
            /* before
                    1
                   / \
                 -1   2
                   \
                    0
            */
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            let (root_wrapper, height_change) = remove_helper(root, 2);
            /* after
                    1           1           0
                   /      ->   /    ->     / \
                 -1           0          -1   1
                   \         /  
                    0      -1      
            */
            println!("After rotations...");
            print_node_tree(root_wrapper.as_ref(), 0);
            if let Some(root) = root_wrapper {
                // should have rotated right after removal
                assert_eq!(root.val, 0);
                assert_eq!(root.balance, 0);
                assert_eq!(height_change, 1);
            } else {
                panic!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_left_child_right_heavy_opposite_neighbor() {
            let root = Some(Box::new(Node::new(0)));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, -1);
            let (root, _) = add_helper(root, 1);
            /* before
                    0
                   / \
                 -1   2
                     / 
                    1
            */
            println!("Before rotations...");
            print_node_tree(root.as_ref(), 0);
            let (root_wrapper, height_change) = remove_helper(root, -1);
            /* after
                    0           0              1
                     \    ->     \     ->     / \
                      2           1          0   2
                     /             \  
                    1               2
            */
            println!("After rotations...");
            print_node_tree(root_wrapper.as_ref(), 0);
            if let Some(root) = root_wrapper {
                // should have rotated left after removal
                assert_eq!(root.val, 1);
                assert_eq!(root.balance, 0);
                assert_eq!(height_change, 1);
            } else {
                panic!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn bigger_tree_test_remove_from_right_half() {
            let mut root = Some(Box::new(Node::new(1)));
            for i in 2..16 {
                (root, _) = add_helper(root, i);
            }
            /* before
                            8
                        /       \
                       4        12
                     /   \    /    \
                    2     6   10   14
                   / \   / \  / \  / \
                  1   3 5   7 9 11 13 15
            
            */
            print_node_tree(root.as_ref(), 0);
            if let Some(node) = root.as_ref() {
                assert_eq!(node.val, 8);
                assert_eq!(node.balance, 0);
            } else {
                panic!("We didn't get a root back from add, something went wrong...")
            }
            for i in 8..16 {
                println!("Removing {}...", i);
                (root, _) = remove_helper(root, i);
                if let Some(node) = root.as_ref() {
                    assert!(node.balance >= -1 && node.balance <= 1);
                    if i < 14 {
                        assert_eq!(7, node.val);
                    } else {
                        assert_eq!(4, node.val);
                    }
                } else {
                    panic!("Remove didn't return a root node, something went wrong");
                }
            }
            print_node_tree(root.as_ref(), 0);
        }

        #[test]
        fn bigger_tree_test_remove_from_left_half() {
            let mut root = Some(Box::new(Node::new(1)));
            for i in 2..16 {
                (root, _) = add_helper(root, i);
            }
            /* before
                            8
                        /       \
                       4        12
                     /   \    /    \
                    2     6   10   14
                   / \   / \  / \  / \
                  1   3 5   7 9 11 13 15
            
            */
            print_node_tree(root.as_ref(), 0);
            if let Some(node) = root.as_ref() {
                assert_eq!(node.val, 8);
                assert_eq!(node.balance, 0);
            } else {
                panic!("We didn't get a root back from add, something went wrong...")
            }
            for i in (1..9).rev() {
                println!("Removing {}...", i);
                (root, _) = remove_helper(root, i);
                if let Some(node) = root.as_ref() {
                    assert!(node.balance >= -1 && node.balance <= 1);
                    if i > 4 {
                        assert_eq!(i-1, node.val);
                    } else if i > 2{
                        assert_eq!(9, node.val);
                    } else {
                        assert_eq!(12, node.val);
                    }
                } else {
                    panic!("Remove didn't return a root node, something went wrong");
                }
            }
            print_node_tree(root.as_ref(), 0);
        }
    }

    mod tree_tests {
        use super::*;

        #[test]
        fn tree_construction() {
            let tree = BinaryTree::<i32>::new();
            assert!(tree.head.is_none());
            assert!(tree.depth == 0);
        }

        #[test]
        fn add_to_empty_tree() {
            let mut tree = BinaryTree::new();
            assert!(tree.head.is_none());
            tree.add(1);
            assert!(!tree.head.is_none());
            assert_eq!(tree.depth, 1);
        }

        #[test]
        fn add_to_tree_failure() {
            let mut tree = BinaryTree::new();
            tree.add(1);
            assert!(tree.contains(1));
            tree.add(1);
            assert_eq!(tree.depth, 1);
            if let Some(head) = tree.head {
                assert!(head.left.is_none());
                assert!(head.right.is_none());
            } else {
                panic!("The tree doesn't have a head, and it should...")
            }
        }

        #[test]
        fn remove_from_empty_tree() {
            let mut tree = BinaryTree::<i32>::new();
            tree.remove(1);
            assert!(tree.head.is_none());
        }

        #[test]
        fn remove_from_tree_success() {
            let mut tree = BinaryTree::new();
            tree.add(1);
            tree.add(8);
            tree.add(2);
            tree.add(10);
            tree.add(-3);
            assert_eq!(tree.depth, 3);
            assert!(tree.contains(1));
            tree.remove(1);
            assert!(!tree.contains(1));
            assert_eq!(tree.depth, 3);
        }

        #[test]
        fn remove_from_tree_failure() {
            let mut tree = BinaryTree::new();
            tree.add(1);
            tree.add(8);
            tree.add(2);
            tree.add(10);
            tree.add(-3);

            assert!(tree.contains(1));
            assert!(tree.contains(8));
            assert!(tree.contains(2));
            assert!(tree.contains(10));
            assert!(tree.contains(-3));

            assert!(!tree.contains(20));
            tree.remove(20);
            
            assert!(tree.contains(1));
            assert!(tree.contains(8));
            assert!(tree.contains(2));
            assert!(tree.contains(10));
            assert!(tree.contains(-3));
        }
        
        #[test]
        fn search_in_empty_tree() {
            let tree = BinaryTree::new();
            assert!(!tree.contains(1));
        }
    }

    mod iterator_tests {
        use super::*;
        use rand::thread_rng;
        use rand::seq::SliceRandom;

        #[test]
        fn initial_iterator_tests() {
            let mut tree = BinaryTree::new();
            let mut items: Vec<i32> = (0..32).collect();
            items.shuffle(&mut thread_rng());
            for i in items {
                tree.add(i);
            }

            let mut last = &-1;
            for i in tree.iter() {
                println!("{}", i);
                assert!(last < i);
                last = i;
            }
        }

        #[test]
        fn empty_tree_iterator() {
            let tree = BinaryTree::<i32>::new();
            for _ in tree.iter() {
                panic!("Shouldn't release anything for empty tree");
            }
        }
    }

    #[test]
    fn search_for_item_in_tree() {
        let root = Some(Box::new(Node::new(8)));
        let (root, _) = add_helper(root, 4);
        let (root, _) = add_helper(root, 10);
        let (root, _) = add_helper(root, 12);
        let (root, _) = add_helper(root, 16);

        assert!(find(&root, 8));
        assert!(find(&root, 4));
        assert!(find(&root, 10));
        assert!(find(&root, 12));
        assert!(find(&root, 16));
    }

    #[test]
    fn search_for_item_not_in_tree() {
        let root = Some(Box::new(Node::new(8)));
        let (root, _) = add_helper(root, 4);
        let (root, _) = add_helper(root, 10);
        let (root, _) = add_helper(root, 12);
        let (root, _) = add_helper(root, 16);

        assert!(!find(&root, 9));
        assert!(!find(&root, 0));
        assert!(!find(&root, 20));
        assert!(!find(&root, 118));
        assert!(!find(&root, 1));
    }
}
