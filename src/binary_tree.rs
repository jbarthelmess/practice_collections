use std::fmt;

struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    balance: i8
}

impl<T: PartialEq + PartialOrd + fmt::Display> Node<T> {
    fn new(item: T) -> Node<T> {
        Node {
            data: item,
            left: None,
            right: None,
            balance: 0
        }
    }

    fn print(&self, indent: &str) {
        println!("{}Node : {}, {}", indent, self.data, self.balance);
        if let Some(child) = &self.left { child.print(&format!("{}    ", indent)); }
        if let Some(child) = &self.right {child.print(&format!("{}    ", indent)); }
    }
}

fn rotate_right<T>(mut root: Box<Node<T>>) -> Box<Node<T>> {
    let old_root_balance = root.balance;
    let mut new_root = root.left.unwrap();
    root.left = new_root.right;
    root.balance = 0;
    new_root.right = Some(root);
    new_root.balance = old_root_balance +2;
    new_root
}

fn rotate_left<T>(mut root: Box<Node<T>>) -> Box<Node<T>> {
    let old_root_balance = root.balance;
    let mut new_root = root.right.unwrap();
    root.right = new_root.left;
    root.balance = 0;
    new_root.left = Some(root);
    new_root.balance = old_root_balance -2;
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

fn add_helper<T: PartialEq + PartialOrd + fmt::Display>(mut parent: Box<Node<T>>, data: T) -> (Box<Node<T>>, i8) {
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

fn search<T: PartialEq + PartialOrd + fmt::Display>(node: &Box<Node<T>>, needle: &T) -> bool {
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

fn find_least<T: PartialEq + PartialOrd + fmt::Display + Clone>(root: &Box<Node<T>>) -> T {
    let mut left = root;
    while let Some(val) = left.left.as_ref() {
        left = val;
    }
    left.data.clone()
}

fn find_greatest<T: PartialEq + PartialOrd + fmt::Display + Clone >(root: &Box<Node<T>>) -> T {
    let mut right = root;
    while let Some(val) = right.right.as_ref() {
        right = val;
    }
    right.data.clone()
}

fn remove_helper<T: PartialEq + PartialOrd + fmt::Display + Clone >(mut parent: Box<Node<T>>, data: &T) -> (Option<Box<Node<T>>>, i8) {
    if parent.data == *data { // we found the item to be removed
        match (parent.left.take(), parent.right.take()) { // we need to figure out what the successor should be, there are four cases
            (None, None) => (None, 1),
            (Some(l), None) => (Some(l), 1),
            (None, Some(r)) => (Some(r), 1),
            (Some(l), Some(r)) => { // if parent has two children, need to find successor,
                if parent.balance == -1 { // get successor from heavier side, decide using balance
                    let new_val = find_greatest(&l);
                    let (new_left, height_change) = remove_helper(l, &new_val);
                    parent.left = new_left;
                    parent.balance += height_change;
                    parent.right = Some(r);
                    parent.data = new_val;
                    (Some(parent), 0)
                } else { // tree is balanced, or right heavy
                    let new_val = find_least(&r);
                    let (new_right, height_change) = remove_helper(r, &new_val);
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
            }else {
                match parent.balance {
                    -2 => { // need to rotate right
                        parent = perform_rotation(parent);
                        (Some(parent), 1)
                    },
                    -1 => {
                        if let Some(_) = parent.left {
                            (Some(parent), 0)
                        } else {
                            (Some(parent), 1)
                        }
                    },
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
                        parent = perform_rotation(parent);
                        (Some(parent), 1)
                    },
                    1 => {
                        if let Some(_) = parent.right {
                            (Some(parent), 0)
                        } else {
                            (Some(parent), 1)
                        }
                    },
                    0 => (Some(parent), 1),
                    _ => unreachable!()
                }
            }
            
        } else { // no values match, return no height change
            (Some(parent), 0)
        }
    }
}

struct BinaryTree<T: PartialEq + PartialOrd + fmt::Display + Clone> {
    head: Option<Box<Node<T>>>
}

impl<T: PartialEq + PartialOrd + fmt::Display + Clone> BinaryTree<T> {
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

    pub fn contains(&mut self, needle: &T) -> bool {
        if let Some(val) = self.head.as_ref() {
            search(val, needle)
        } else {
            false
        }
    }

    pub fn print(&self) {
        if let Some(val) = &self.head { val.print("");}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_test() {
        let mut tree = BinaryTree::<i32>::new();
        tree.add(1);
        tree.add(2);
        tree.add(3);
        tree.print();
    }

    #[test]
    fn second_test() {
        let mut tree = BinaryTree::<i32>::new();
        for i in 0..10 {
            tree.add(i);
        }
        tree.print();
    }

    #[test]
    fn search_test() {
        let mut tree = BinaryTree::<i32>::new();
        for i in 0..15 {
            tree.add(i);
        }
        for i in 0..15 {
            assert!(tree.contains(&i));
        }
        assert!(!tree.contains(&15));
    }

    #[test]
    fn remove() {
        let mut tree = BinaryTree::<i32>::new();
        for i in 1..32 {
            tree.add(i);
        }
        tree.remove(22);
        tree.remove(17);
        tree.remove(19);
        println!("After removal...\n");
        tree.print();
        tree.remove(18);

        println!("After requisite rotations...\n");
        tree.print();
    }
}