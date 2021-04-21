struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

struct BinaryTree<T: PartialEq + PartialOrd> {
    head: Option<Box<Node<T>>>,
    size: usize
}

impl<T: PartialEq + PartialOrd> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree {
            head: None,
            size: 0
        }
    }

    fn add_helper(parent: &mut Box<Node<T>>, item: T) -> bool{ // recursive add helper function to add to the tree
        let val = &parent.data;
        if item == *val { // if there is a duplicate, we return without adding anything
            return false; 
        } else if item > *val { // follow right branch
            match parent.right {
                Some(ref mut node) => {
                    return BinaryTree::add_helper(node, item);
                },
                None => {
                    parent.right = Some(Box::new(Node{data: item, left: None, right: None}));
                    return true;
                }
            }
        } else { // follow left branch
            match parent.left {
                Some(ref mut node) => {
                    return BinaryTree::add_helper(node, item);
                },
                None => {
                    parent.left = Some(Box::new(Node{data: item, left: None, right: None}));
                    return true;
                }
            }
        }
    }

    pub fn add(&mut self, item: T) {
        match self.head {
            Some(ref mut val) => {
                let check = BinaryTree::add_helper(val, item);
                if check { self.size+= 1; }
            },
            None => {
                self.head = Some(Box::new(Node{data: item, left: None, right: None}));
                self.size+= 1;
            }
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_to_tree() {
        let mut tree = BinaryTree::<i32>::new();

        // new tree should be empty
        assert_eq!(0, tree.size());

        // adding one item should increase size by one
        tree.add(1);
        assert_eq!(1, tree.size());
        
        // adding more items
        tree.add(2);
        tree.add(3);
        assert_eq!(3, tree.size());

        // adding a duplicate shouldn't increase the size of the tree
        tree.add(3);
        assert_eq!(3, tree.size());
    }
}