struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    balance: i8
}

impl<T: Ord> Node<T> {
    fn new(item: T) -> Self {
        Node {
            data: item,
            left: None,
            right: None,
            balance: 0
        }
    }
}

pub struct BinaryTree<T: Ord> {
    head: Option<Box<Node<T>>>
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree {
            head: None
        }
    }

    pub fn add(&mut self, item: T) {
        match self.head.take() {
            Some(val) => self.head = Some(add_helper(val, item).0),
            None => self.head = Some(Box::new(Node::new(item)))
        }
    }

    pub fn remove(&mut self, item: T) {
        if let Some(val) = self.head.take() {
            self.head = remove_helper(val, &item).0;
        }
    }

    pub fn contains(&self, needle: &T) -> bool {
        if let Some(val) = self.head.as_ref() {
            search(val, needle)
        } else {
            false
        }
    }

    pub fn iter(&self) -> TreeIterator<T> {
        TreeIterator::new(self)
    }
}

/**
 * This function does the work of adding Nodes to the tree, and realigning if necessary
 * 
 * A recursive function that returns the new child to the parent after the addition of the item, 
 * and a number indicating how the height of the sub tree has changed. 
 * 
 * Increases to the height of the left subtree will be treated as negative in the balance calculations,
 * and increases to the height of the right subtree will be treated as positive
 */
fn add_helper<T: Ord>(mut parent: Box<Node<T>>, data: T) -> (Box<Node<T>>, i8) {
    if data == parent.data { // if it's already in the tree, delete it
        (parent, 0)
    } else if data > parent.data { // follow the right child down
        if let Some(node) = parent.right { // if there is a right child, recursively add down the tree
            let (right, height_change) = add_helper(node, data);
            parent.balance += height_change;
            parent.right = Some(right);
            if parent.balance == 2 { // if the parent is now too right heavy, we need to rotate left
                parent = perform_rotation(parent);
                return (parent, 0);
            } else { // otherwise just return the parent and height_change info
                return (parent, height_change);
            }
        } else { // otherwise set right child as new data inserted
            parent.right = Some(Box::new(Node::new(data)));
            parent.balance += 1;
            if let Some(_) = parent.left {
                return (parent, 0);
            } else {
                return (parent, 1);
            }
        }
    } else { // follow the left child down
        if let Some(node) = parent.left { // if there is a left child, recursively add down the tree
            let (left, height_change) = add_helper(node, data);
            parent.balance -= height_change;
            parent.left = Some(left);
            if parent.balance == -2 {
                parent = perform_rotation(parent);
                return (parent, 0);
            } else {
                return (parent, height_change);
            }
        } else {
            parent.left = Some(Box::new(Node::new(data)));
            parent.balance -= 1;
            if let Some(_) = parent.right {
                return (parent, 0);
            } else {
                return (parent, 1);
            }
        }
    }
}

/**
 * This function does the work of removing Nodes from the tree, and realigning if necessary
 * 
 * A recursive function that returns the new child to the parent after the removal of the item,
 * along with a number indicating the height change of the subtree
 */
fn remove_helper<T: Ord>(mut parent: Box<Node<T>>, data: &T) -> (Option<Box<Node<T>>>, i8) {
    if parent.data == *data { // we found the item to be removed
        match (parent.left.take(), parent.right.take()) { // we need to figure out what the successor should be, there are four cases
            (None, None) => (None, 1),
            (Some(l), None) => (Some(l), 1),
            (None, Some(r)) => (Some(r), 1),
            (Some(l), Some(r)) => { // if parent has two children, need to find successor,
                if parent.balance == -1 { // get successor from heavier side, decide using balance
                    let (new_left, new_val, height_change) = remove_greatest(l);
                    parent.left = new_left;
                    parent.balance += height_change;
                    parent.right = Some(r);
                    parent.data = new_val;
                    (Some(parent), 0)
                } else { // tree is balanced, or right heavy
                    let (new_right, new_val, height_change) = remove_least(r);
                    parent.right = new_right;
                    parent.balance -= height_change;
                    parent.left = Some(l);
                    parent.data = new_val;
                    (Some(parent), 0)
                }
            }
        }
    } else if *data > parent.data { // search the right tree
        if let Some(val) = parent.right { // check for child
            let (updated_right, height_change) = remove_helper(val, data);
            parent.right = updated_right;
            parent.balance -= height_change;
            if height_change == 0 {
                (Some(parent), 0)
            } else {
                match parent.balance {
                    -2 => { // need to rotate right
                        let left = parent.left.take().unwrap();
                        let left_balance = left.balance;
                        parent.left = Some(left);
                        parent = perform_rotation(parent);
                        if left_balance == 0 {
                            (Some(parent), 0)
                        } else {
                            (Some(parent), 1)
                        }
                    },
                    -1 => (Some(parent), 0),
                    0 => (Some(parent), 1),
                    _ => unreachable!()
                }
            }
        } else { // no values match, return no height change
            (Some(parent), 0)
        }
    } else { // search the left tree
        if let Some(val) = parent.left { // check for child
            let (updated_left, height_change) = remove_helper(val, data);
            parent.left = updated_left;
            parent.balance += height_change;
            if height_change == 0 {
                (Some(parent), 0)
            } else {
                match parent.balance {
                    2 => { // need to rotate left
                        let right = parent.right.take().unwrap();
                        let right_balance = right.balance;
                        parent.right = Some(right);
                        parent = perform_rotation(parent);
                        if right_balance == 0 {
                            (Some(parent), 0)
                        } else {
                            (Some(parent), 1)
                        }
                    },
                    1 => (Some(parent), 0),
                    0 => (Some(parent), 1),
                    _ => unreachable!()
                }
            }
        } else { // no values match, return no height change
            (Some(parent), 0)
        }
    }
}

// TREE ROTATION FUNCTIONS 

fn rotate_right<T>(mut root: Box<Node<T>>) -> Box<Node<T>> {
    assert!(root.balance < 0);
    let old_root_balance = root.balance;
    let mut new_root = root.left.unwrap();
    let new_root_balance = new_root.balance;
    root.left = new_root.right;
    root.balance = old_root_balance +1 - new_root_balance;
    new_root.right = Some(root);
    if old_root_balance == -1 && new_root_balance == 0 {
        new_root.balance = old_root_balance +2;
    } else {
        new_root.balance = old_root_balance +3 + new_root_balance;
    }
    new_root
}

fn rotate_left<T>(mut root: Box<Node<T>>) -> Box<Node<T>> {
    assert!(root.balance > 0);
    let old_root_balance = root.balance;
    let mut new_root = root.right.unwrap();
    let new_root_balance = new_root.balance;
    root.right = new_root.left;
    root.balance = old_root_balance -1 - new_root_balance;
    new_root.left = Some(root);
    if old_root_balance == 1 && new_root_balance == 0 {
        new_root.balance = old_root_balance -2;
    } else {
        new_root.balance = old_root_balance -3 + new_root_balance;
    }
    
    new_root
}

fn perform_rotation<T>(mut root: Box<Node<T>>) -> Box<Node<T>> {
    if root.balance < 0 {  // rotate right
        if let Some(mut left) = root.left{
            if left.balance > 0 {
                left = perform_rotation(left);
            }
            root.left = Some(left);
            root = rotate_right(root);
        } else {
            panic!("AVL Tree Broken");
        }
        root
    } else if root.balance > 0 { // rotate left
        if let Some(mut right) = root.right {
            if right.balance < 0 {
                right = perform_rotation(right);
            }
            root.right = Some(right);
            root = rotate_left(root);
        } else {
            panic!("AVL Tree Broken");
        }
        root
    } else {
        root
    }
}

pub struct TreeIterator<'a, T:'a+Ord> {
    stack: crate::stack::Stack<&'a Box<Node<T>>>
}

impl<'a, T:'a+Ord> TreeIterator<'a, T> {
    fn new(tree: &'a BinaryTree<T>) -> Self {
        let mut stack = crate::stack::Stack::new();
        let mut begin = tree.head.as_ref();
        while let Some(left) = begin {
            stack.push(left);
            begin = left.left.as_ref();
        }
        TreeIterator {/*tree, */stack}
    }
}

impl<'a, T:'a+Ord> Iterator for TreeIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let release = self.stack.pop();
        if let Some(node) = release {
            let mut begin = &node.right;
            while let Some(child) = begin {
                self.stack.push(child);
                begin = &child.left;
            }
            Some(&node.data)
        } else {
           None 
        }
    }
}



fn search<T: Ord>(node: &Box<Node<T>>, needle: &T) -> bool {
    if node.data == *needle {
        true
    } else if node.data > *needle {
        if let Some(val) = node.left.as_ref() {
            search(val, needle)
        } else {
            false
        }
    } else {
        if let Some(val) = node.right.as_ref() {
            search(val, needle)
        } else {
            false
        }
    }
}

fn remove_least<T>(mut root: Box<Node<T>>) -> (Option<Box<Node<T>>>, T, i8) {
    if let Some(val) = root.left {
        let (new_child, big, height_change) = remove_least(val);
        root.left = new_child;
        root.balance += height_change;
        if height_change == 0 {
            (Some(root), big,  0)
        } else {
            match root.balance {
                2 => { // need to rotate left
                    let right = root.right.take().unwrap();
                    let right_balance = right.balance;
                    root.right = Some(right);
                    root = perform_rotation(root);
                    if right_balance == 0 {
                        (Some(root), big, 0)
                    } else {
                        (Some(root), big, 1)
                    }
                },
                1 => (Some(root), big, 0),
                0 => (Some(root), big, 1),
                _ => unreachable!("If the node was balanced before the removal, 2, 1, or 0 should be the only balance values after removing from the left child, got something else...")
            }
        }
    } else { // we've found the lowest value in the tree
        match (root.left.take(), root.right.take()) { // we need to figure out what the successor should be, there are four cases
            (None, None) => (None, root.data, 1),
            (None, Some(r)) => (Some(r), root.data, 1),
            (Some(_), None) => unreachable!("Didn't actually find the least value, something went wrong..."),
            (Some(_), Some(_)) => unreachable!("Didn't actually find the least value, something went wrong...")
        }
    }
}

fn remove_greatest<T>(mut root: Box<Node<T>>) -> (Option<Box<Node<T>>>, T, i8) {
    if let Some(val) = root.right {
        let (new_child, big, height_change) = remove_greatest(val);
        root.right = new_child;
        root.balance -= height_change;
        if height_change == 0 {
            (Some(root), big,  0)
        } else {
            match root.balance {
                -2 => { // need to rotate right
                    let left = root.left.take().unwrap();
                    let left_balance = left.balance;
                    root.left = Some(left);
                    root = perform_rotation(root);
                    if left_balance == 0 {
                        (Some(root), big, 0)
                    } else {
                        (Some(root), big, 1)
                    }
                },
                -1 => (Some(root), big, 0),
                0 => (Some(root), big, 1),
                _ => unreachable!("If the node was balanced before the removal, -2, -1, or 0 should be the only balance values after removing from the right child, got something else...")
            }
        }
    } else { // we've found the greatest value in the tree
        match (root.left.take(), root.right.take()) { // we need to figure out what the successor should be, there are four cases
            (None, None) => (None, root.data, 1),
            (Some(l), None) => (Some(l), root.data, 1),
            (None, Some(_)) => unreachable!("Didn't actually find the greatest value, something went wrong..."),
            (Some(_), Some(_)) => unreachable!("Didn't actually find the greatest value, something went wrong...")
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Display;

    fn print_node_tree<T: Display>(tree: &Box<Node<T>>, depth: usize) {
        if let Some(node) = &tree.left {
            print_node_tree(node, depth+1);
        }
        println!("{}{}:{}", " ".repeat(2*depth), tree.data, tree.balance);
        if let Some(node) = &tree.right {
            print_node_tree(node, depth+1);
        }
    }
    
    // Test Node Addition first
    mod add_tests {
        use super::*;

        #[test]
        fn node_add_right() {
            let root = Box::new(Node::new(1));
            assert!(root.left.is_none());
            assert!(root.right.is_none());
            assert_eq!(root.balance, 0);

            let (root, height_change) = add_helper(root, 2);
            assert!(root.left.is_none());
            assert!(!root.right.is_none());
            assert_eq!(height_change, 1);
            assert_eq!(root.balance, 1);
        }

        #[test]
        fn node_add_left() {
            let root = Box::new(Node::new(1));
            assert!(root.left.is_none());
            assert!(root.right.is_none());
            assert_eq!(root.balance, 0);

            let (root, height_change) = add_helper(root, 0);
            assert!(!root.left.is_none());
            assert!(root.right.is_none());
            assert_eq!(height_change, 1);
            assert_eq!(root.balance, -1);
        }

        #[test]
        fn node_add_duplicate() {
            let root = Box::new(Node::new(1));
            assert!(root.left.is_none());
            assert!(root.right.is_none());
            assert_eq!(root.balance, 0);

            let (root, height_change) = add_helper(root, 1);
            assert!(root.left.is_none());
            assert!(root.right.is_none());
            assert_eq!(height_change, 0);
            assert_eq!(root.balance, 0);
        }

        #[test]
        fn simple_left_rotation() {
            let root = Box::new(Node::new(1));
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
            assert_ne!(root.data, 1);
            assert_eq!(height_change, 0);
            assert_eq!(root.balance, 0);
        }

        #[test]
        fn simple_right_rotation() {
            let root = Box::new(Node::new(3));
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
            assert_ne!(root.data, 3);
            assert_eq!(height_change, 0);
            assert_eq!(root.balance, 0);
        }

        #[test]
        fn right_rotation_left_heavy_left_child() {
            let root = Box::new(Node::new(5));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 7);
            let (root, _) = add_helper(root, 1);
            let (root, _) = add_helper(root, 3);
            assert_eq!(root.data, 5);
            assert_eq!(root.balance, -1);
            println!("Before rotations...");
            print_node_tree(&root, 0);
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
            print_node_tree(&root, 0);
            assert_ne!(root.data, 5);
            assert_eq!(root.data, 2);
            assert_eq!(height_change, 0);
            assert_eq!(root.balance, 0);
        }

        #[test]
        fn right_rotation_right_heavy_left_child() {
            let root = Box::new(Node::new(5));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 7);
            let (root, _) = add_helper(root, 1);
            let (root, _) = add_helper(root, 3);
            assert_eq!(root.data, 5);
            assert_eq!(root.balance, -1);
            println!("Before rotations...");
            print_node_tree(&root, 0);
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
            print_node_tree(&root, 0);
            assert_ne!(root.data, 5);
            assert_eq!(root.data, 3);
            assert_eq!(height_change, 0);
            assert_eq!(root.balance, 0);
        }

        #[test]
        fn right_rotation_left_heavy_left_child_no_right_child() {
            let root = Box::new(Node::new(5));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 7);
            let (root, _) = add_helper(root, 1);
            assert_eq!(root.data, 5);
            assert_eq!(root.balance, -1);
            println!("Before rotations...");
            print_node_tree(&root, 0);
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
            print_node_tree(&root, 0);
            assert_eq!(root.data, 5);
            assert_eq!(height_change, 0);
            assert_eq!(root.balance, -1);
        }

        #[test]
        fn right_rotation_right_heavy_left_child_no_left_child() {
            let root = Box::new(Node::new(5));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 7);
            let (root, _) = add_helper(root, 3);
            assert_eq!(root.data, 5);
            assert_eq!(root.balance, -1);
            println!("Before rotations...");
            print_node_tree(&root, 0);
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
            print_node_tree(&root, 0);
            assert_eq!(root.data, 5);
            assert_eq!(height_change, 0);
            assert_eq!(root.balance, -1);
        }

        #[test]
        fn left_rotation_left_heavy_right_child() {
            let root = Box::new(Node::new(4));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 7);
            let (root, _) = add_helper(root, 6);
            let (root, _) = add_helper(root, 8);
            assert_eq!(root.data, 4);
            assert_eq!(root.balance, 1);
            println!("Before rotations...");
            print_node_tree(&root, 0);
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
            print_node_tree(&root, 0);
            assert_ne!(root.data, 4);
            assert_eq!(root.data, 6);
            assert_eq!(height_change, 0);
            assert_eq!(root.balance, 0);
        }

        #[test]
        fn left_rotation_right_heavy_right_child() {
            let root = Box::new(Node::new(4));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 6);
            let (root, _) = add_helper(root, 5);
            let (root, _) = add_helper(root, 7);
            assert_eq!(root.data, 4);
            assert_eq!(root.balance, 1);
            println!("Before rotations...");
            print_node_tree(&root, 0);
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
            print_node_tree(&root, 0);
            assert_ne!(root.data, 4);
            assert_eq!(root.data, 6);
            assert_eq!(height_change, 0);
            assert_eq!(root.balance, 0);
        }

        #[test]
        fn left_rotation_left_heavy_right_child_no_right_child() {
            let root = Box::new(Node::new(4));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 7);
            let (root, _) = add_helper(root, 6);
            assert_eq!(root.data, 4);
            assert_eq!(root.balance, 1);
            println!("Before rotations...");
            print_node_tree(&root, 0);
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
            print_node_tree(&root, 0);
            assert_eq!(root.data, 4);
            assert_eq!(height_change, 0);
            assert_eq!(root.balance, 1);
        }

        #[test]
        fn left_rotation_right_heavy_right_child_no_left_child() {
            let root = Box::new(Node::new(5));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 7);
            let (root, _) = add_helper(root, 8);
            assert_eq!(root.data, 5);
            assert_eq!(root.balance, 1);
            println!("Before rotations...");
            print_node_tree(&root, 0);
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
            print_node_tree(&root, 0);
            assert_eq!(root.data, 5);
            assert_eq!(height_change, 0);
            assert_eq!(root.balance, 1);
        }
    }

    mod remove_tests {
        use super::*;

        #[test]
        fn remove_root() {
            let root = Box::new(Node::new(4));
            let (root, height_change) = remove_helper(root, &4);
            assert!(root.is_none());
            assert_eq!(height_change, 1)
        }

        #[test]
        fn remove_left() {
            let root = Box::new(Node::new(1));
            let (root, _) = add_helper(root, 0);
            assert!(!root.left.is_none());
            let (root_wrapper, height_change) = remove_helper(root, &0);
            if let Some(root) = root_wrapper {
                assert!(root.left.is_none());
                assert_eq!(root.data, 1);
                assert_eq!(height_change, 1);
            } else {
                unreachable!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_right() {
            let root = Box::new(Node::new(1));
            let (root, _) = add_helper(root, 2);
            assert!(!root.right.is_none());
            let (root_wrapper, height_change) = remove_helper(root, &2);
            if let Some(root) = root_wrapper {
                assert!(root.right.is_none());
                assert_eq!(root.data, 1);
                assert_eq!(height_change, 1);
            } else {
                unreachable!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_root_with_left_child() {
            let root = Box::new(Node::new(1));
            let (root, _) = add_helper(root, 0);
            assert!(!root.left.is_none());
            let (root_wrapper, height_change) = remove_helper(root, &1);
            if let Some(root) = root_wrapper {
                assert!(root.left.is_none());
                assert_eq!(root.data, 0);
                assert_eq!(height_change, 1);
            } else {
                unreachable!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_root_with_right_child() {
            let root = Box::new(Node::new(1));
            let (root, _) = add_helper(root, 2);
            assert!(!root.right.is_none());
            let (root_wrapper, height_change) = remove_helper(root, &1);
            if let Some(root) = root_wrapper {
                assert!(root.right.is_none());
                assert_eq!(root.data, 2);
                assert_eq!(height_change, 1);
            } else {
                unreachable!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_root_with_two_children() {
            let root = Box::new(Node::new(1));
            let (root, _) = add_helper(root, 2);
            let (root, _) = add_helper(root, 0);
            /*before
                    1
                   / \
                  0   2
             */
            assert!(!root.right.is_none());
            assert!(!root.left.is_none());
            let (root_wrapper, height_change) = remove_helper(root, &1);
            if let Some(root) = root_wrapper {
                // biases towards pulling from the right tree in a balanced tree
                assert!(root.right.is_none());
                assert_eq!(root.data, 2);
                assert_eq!(height_change, 0);
            } else {
                unreachable!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_root_with_two_children_left_heavy() {
            let root = Box::new(Node::new(1));
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
            let (root_wrapper, height_change) = remove_helper(root, &1);
            /* after
                    0
                   / \
                 -1   2
                 /
               -2  
            */
            if let Some(root) = root_wrapper {
                // pulls from the left when it is left heavy
                assert_eq!(root.data, 0);
                assert_eq!(height_change, 0);
                print_node_tree(&root, 0);
            } else {
                unreachable!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_right_child_left_heavy() {
            let root = Box::new(Node::new(1));
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
            let (root_wrapper, height_change) = remove_helper(root, &2);
            /* after
                    1            -1
                   /      ->     / \
                 -1            -2   1
                 / \               /
               -2   0             0
            */
            if let Some(root) = root_wrapper {
                // should have rotated right after removal
                assert_eq!(root.data, -1);
                assert_eq!(root.balance, 1);
                assert_eq!(height_change, 0);
                print_node_tree(&root, 0);
            } else {
                unreachable!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_left_child_right_heavy() {
            let root = Box::new(Node::new(0));
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
            let (root_wrapper, height_change) = remove_helper(root, &-1);
            /* after
                    0             2
                     \    ->     / \
                      2         0   3
                     / \         \      
                    1   3         1
            */
            if let Some(root) = root_wrapper {
                // should have rotated right after removal
                assert_eq!(root.data, 2);
                assert_eq!(root.balance, -1);
                assert_eq!(height_change, 0);
                print_node_tree(&root, 0);
            } else {
                unreachable!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_right_child_left_heavy_no_neighbor() {
            let root = Box::new(Node::new(1));
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
            let (root_wrapper, height_change) = remove_helper(root, &2);
            /* after
                    1            -1
                   /      ->     / \
                 -1            -2   1
                 /            
               -2                
            */
            if let Some(root) = root_wrapper {
                // should have rotated right after removal
                assert_eq!(root.data, -1);
                assert_eq!(root.balance, 0);
                assert_eq!(height_change, 1);
                print_node_tree(&root, 0);
            } else {
                unreachable!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_left_child_right_heavy_no_neighbor() {
            let root = Box::new(Node::new(0));
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
            let (root_wrapper, height_change) = remove_helper(root, &-1);
            /* after
                    0             2
                     \    ->     / \
                      2         0   3
                       \               
                        3         
            */
            if let Some(root) = root_wrapper {
                // should have rotated right after removal
                assert_eq!(root.data, 2);
                assert_eq!(root.balance, 0);
                assert_eq!(height_change, 1);
                print_node_tree(&root, 0);
            } else {
                unreachable!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_right_child_left_heavy_opposite_neighbor() {
            let root = Box::new(Node::new(1));
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
            let (root_wrapper, height_change) = remove_helper(root, &2);
            /* after
                    1           1           0
                   /      ->   /    ->     / \
                 -1           0          -1   1
                   \         /  
                    0      -1      
            */
            if let Some(root) = root_wrapper {
                // should have rotated right after removal
                assert_eq!(root.data, 0);
                assert_eq!(root.balance, 0);
                assert_eq!(height_change, 1);
                print_node_tree(&root, 0);
            } else {
                unreachable!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn remove_left_child_right_heavy_opposite_neighbor() {
            let root = Box::new(Node::new(0));
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
            let (root_wrapper, height_change) = remove_helper(root, &-1);
            /* after
                    0           0              1
                     \    ->     \     ->     / \
                      2           1          0   2
                     /             \  
                    1               2
            */
            if let Some(root) = root_wrapper {
                // should have rotated right after removal
                assert_eq!(root.data, 1);
                assert_eq!(root.balance, 0);
                assert_eq!(height_change, 1);
                print_node_tree(&root, 0);
            } else {
                unreachable!("Remove didn't work properly, no root was returned from the tree");
            }
        }

        #[test]
        fn bigger_tree_test_remove_from_right_half() {
            let mut root = Box::new(Node::new(1));
            for i in 2..16 {
                root = add_helper(root, i).0;
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
            assert_eq!(root.data, 8);
            assert_eq!(root.balance, 0);
            print_node_tree(&root, 0);
            for i in 8..16 {
                if i < 13 {
                    assert_eq!(i, root.data);
                } else if i < 15{
                    assert_eq!(7, root.data);
                } else {
                    assert_eq!(4, root.data);
                }
                println!("Removing {}...", i);
                let (root_wrapper, _) = remove_helper(root, &i);
                if let Some(new_root) = root_wrapper {
                    root = new_root;
                    assert!(root.balance >= -1 && root.balance <= 1);

                } else {
                    unreachable!("Remove didn't return a root node, something went wrong");
                }
            }
            print_node_tree(&root, 0);
        }

        #[test]
        fn bigger_tree_test_remove_from_left_half() {
            let mut root = Box::new(Node::new(1));
            for i in 2..16 {
                root = add_helper(root, i).0;
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
            assert_eq!(root.data, 8);
            assert_eq!(root.balance, 0);
            print_node_tree(&root, 0);
            for i in (1..9).rev() {
                if i == 8 {
                    assert_eq!(i, root.data);
                } else if i > 1{
                    assert_eq!(9, root.data);
                } else {
                    assert_eq!(12, root.data);
                }
                println!("Removing {}...", i);
                let (root_wrapper, _) = remove_helper(root, &i);
                if let Some(new_root) = root_wrapper {
                    root = new_root;
                    assert!(root.balance >= -1 && root.balance <= 1);
                } else {
                    unreachable!("Remove didn't return a root node, something went wrong");
                }
            }
            print_node_tree(&root, 0);
        }
    }

    mod tree_tests {
        use super::*;

        #[test]
        fn tree_construction() {
            let tree = BinaryTree::<i32>::new();
            assert!(tree.head.is_none());
        }

        #[test]
        fn add_to_empty_tree() {
            let mut tree = BinaryTree::new();
            assert!(tree.head.is_none());
            tree.add(1);
            assert!(!tree.head.is_none());
        }

        #[test]
        fn add_to_tree_failure() {
            let mut tree = BinaryTree::new();
            tree.add(1);
            assert!(tree.contains(&1));
            tree.add(1);
            if let Some(head) = tree.head {
                assert!(head.left.is_none());
                assert!(head.right.is_none());
            } else {
                unreachable!("The tree doesn't have a head, and it should...")
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

            assert!(tree.contains(&1));
            tree.remove(1);
            assert!(!tree.contains(&1));
        }

        #[test]
        fn remove_from_tree_failure() {
            let mut tree = BinaryTree::new();
            tree.add(1);
            tree.add(8);
            tree.add(2);
            tree.add(10);
            tree.add(-3);

            assert!(tree.contains(&1));
            assert!(tree.contains(&8));
            assert!(tree.contains(&2));
            assert!(tree.contains(&10));
            assert!(tree.contains(&-3));

            assert!(!tree.contains(&20));
            tree.remove(20);
            
            assert!(tree.contains(&1));
            assert!(tree.contains(&8));
            assert!(tree.contains(&2));
            assert!(tree.contains(&10));
            assert!(tree.contains(&-3));
        }
        
        #[test]
        fn search_in_empty_tree() {
            let tree = BinaryTree::<i32>::new();
            assert!(!tree.contains(&1));
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
                assert!(last < i);
                last = i;
            }
        }

        #[test]
        fn empty_tree_iterator() {
            let tree = BinaryTree::<i32>::new();
            for _ in tree.iter() {
                unreachable!("Shouldn't release anything for empty tree");
            }
        }
    }

    #[test]
    fn search_for_item_in_tree() {
        let root = Box::new(Node::new(8));
        let (root, _) = add_helper(root, 4);
        let (root, _) = add_helper(root, 10);
        let (root, _) = add_helper(root, 12);
        let (root, _) = add_helper(root, 16);

        assert!(search(&root, &8));
        assert!(search(&root, &4));
        assert!(search(&root, &10));
        assert!(search(&root, &12));
        assert!(search(&root, &16));
    }

    #[test]
    fn search_for_item_not_in_tree() {
        let root = Box::new(Node::new(8));
        let (root, _) = add_helper(root, 4);
        let (root, _) = add_helper(root, 10);
        let (root, _) = add_helper(root, 12);
        let (root, _) = add_helper(root, 16);

        assert!(!search(&root, &9));
        assert!(!search(&root, &0));
        assert!(!search(&root, &20));
        assert!(!search(&root, &118));
        assert!(!search(&root, &1));
    }
    
    
}