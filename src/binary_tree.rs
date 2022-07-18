use std::collections::VecDeque;

struct Node<K, V> {
    order: usize,
    keys: VecDeque<K>,
    values: VecDeque<V>,
    children: VecDeque<Option<Box<Node<K,V>>>>,
    is_leaf: bool
}

impl<K: Ord, V> Node<K, V> {
    fn new(order: usize, is_leaf: bool) -> Self {
        if order < 2 {
            panic!("Cannot have a tree with an order less than two");
        }
        Self {
            order,
            keys: VecDeque::with_capacity(order),
            values: VecDeque::with_capacity(order),
            children:VecDeque::with_capacity(order + 1), 
            is_leaf
        }
    }

    // when this function returns a new sibling the sibling will contain
    // the larger half of the values.
    fn add(&mut self, set: (K, V), new_child: Option<Box<Node<K,V>>>) -> Option<(Box<Node<K, V>>, (K, V))> {
        self.add_key_value_child(set, new_child);
        if self.keys.len() == self.order {
            // need to split the node and return the new node and the parent
            
            let mut sibling = Box::new(Node::new(self.order, self.is_leaf));
            for _ in 0..self.keys.len()/2 {
                let key = self.keys.pop_back().unwrap();
                let value = self.values.pop_back().unwrap();
                let child = self.children.pop_back().unwrap();
                sibling.keys.push_front(key);
                sibling.values.push_front(value);
                sibling.children.push_front(child);
            }
            let parent_key = self.keys.pop_back().unwrap();
            let parent_value = self.values.pop_back().unwrap();
            let extra_child = self.children.pop_back().unwrap();
            sibling.children.push_front(extra_child);
            Some((sibling, (parent_key, parent_value)))
        } else {
            None
        }
    }

    fn find_key_val_pos(&self, key: &K) -> usize {
        let mut pos = self.keys.len();
        for (i, val) in self.keys.iter().enumerate() {
            if key <= val {
                pos = i;
                break;
            }
        }
        pos
    }

    fn add_key_value_child(&mut self, set: (K, V), new_child: Option<Box<Node<K,V>>>) {
        let pos = self.find_key_val_pos(&set.0);
        if pos < self.keys.len() && self.keys[pos] == set.0 {
            self.values[pos] = set.1;
        } else {
            if self.children.len() > self.keys.len() {
                self.children.insert(pos+1, new_child);
            } else {
                self.children.insert(pos, new_child);
            }
            self.keys.insert(pos, set.0);
            self.values.insert(pos, set.1);
        }
    }
}

fn add_helper<K: Ord, V>(root: Option<Box<Node<K,V>>>, set: (K, V), order: usize) -> Option<Box<Node<K,V>>> {
    if let Some(mut node) = root {
        let mut stack = Vec::new();
        // push parents onto stack to deal with them as needed
        while !node.is_leaf {
            let pos = node.find_key_val_pos(&set.0);
            let child = node.children[pos].take().unwrap();
            stack.push(node);
            node = child;
        }

        // node is now a leaf node, so we attempt to add the new set to it
        let mut result = node.add(set, None);

        while let Some((new_node, (root_key, root_value))) = result {
            if stack.is_empty() {
                let mut new_root = Box::new(Node::new(order, false));
                new_root.add_key_value_child((root_key, root_value), Some(node));
                new_root.children.push_back(Some(new_node));
                result = None;
                node = new_root;
            } else {
                let mut parent = stack.pop().unwrap();
                for i in 0..parent.children.len() {
                    if parent.children[i].is_none() {
                        parent.children[i] = Some(node);
                        break;
                    }
                }
                node = parent;
                result = node.add((root_key, root_value), Some(new_node));
            }
        }

        // might need to replace all the taken children
        while let Some(mut parent) = stack.pop() {
            for i in 0..parent.children.len() {
                if parent.children[i].is_none() {
                    parent.children[i] = Some(node);
                    break;
                }
            }
            node = parent;
        }
        Some(node)
    } else {
        let mut root = Box::new(Node::new(order, true));
        if let Some(_) = root.add(set, None) {
            unreachable!("Just made a new Node, should be able to add to it with out kicking up a new node");
        }
        root.children.push_back(None);
        Some(root)
    }
}

pub struct BTree<K,V> {
    root: Option<Box<Node<K,V>>>,
    order: usize
}

impl<K: Ord, V> BTree<K,V> {
    pub fn new(order: usize) -> Self {
        Self {
            root: None,
            order
        }
    }

    pub fn insert(&mut self, set: (K, V)) {
        self.root = add_helper(self.root.take(), set, self.order);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    fn print_tree<K: Debug,V: Debug>(root: &Option<Box<Node<K,V>>>, depth: usize) {
        if let Some(node) = root {
            for i in 0..node.keys.len() {
                println!("{}{:?}: {:?}", " ".repeat(4*depth), node.keys[i], node.values[i]);
            }
            if !node.is_leaf {
                println!("{}Node has {} children", " ".repeat(4*depth), node.children.len());
                for (i, child) in node.children.iter().enumerate() {
                    println!("{}child {}", " ".repeat(4*depth), i+1);
                    print_tree(child, depth+1);
                    println!("");
                }
            }
        }
    }

    #[test]
    fn add_test_1() {
        let mut tree = BTree::new(5);
        assert!(tree.root.is_none());
        tree.insert((1, "First item"));
        assert!(!tree.root.is_none());
        tree.insert((3, "Added this second"));
        tree.insert((-1, "Added this third"));
        print_tree(&tree.root, 0);
    }

    #[test]
    fn add_test_2() {
        let mut tree = BTree::new(5);
        tree.insert((1, "First item"));
        tree.insert((2, "Second item"));
        tree.insert((3, "This item should end up in the root"));
        tree.insert((4, "First item, in right child"));
        tree.insert((5, "Second item, in right child"));
        assert!(!tree.root.is_none());
        if let Some(node) = &tree.root {
            assert_ne!(node.keys.len(), 5);
        }
        print_tree(&tree.root, 0);

        // add more nodes
        tree.insert((6, "should be added to right child, and eventually end up in root"));
        tree.insert((0, "should be added to left child"));
        tree.insert((7, "this will fill right"));
        tree.insert((8, "should cause 6 to bump up to root node"));
        print_tree(&tree.root, 0);
    }

    #[test]
    fn add_test_3() {
        let mut tree = BTree::new(5);
        tree.insert((1, "First item"));
        tree.insert((2, "Second item"));
        tree.insert((3, "This item should end up in the root"));
        tree.insert((4, "First item, in right child"));
        tree.insert((5, "Second item, in right child"));
        assert!(!tree.root.is_none());
        if let Some(node) = &tree.root {
            assert_ne!(node.keys.len(), 5);
        }
        print_tree(&tree.root, 0);

        // add more nodes
        tree.insert((6, "should be added to right child, and eventually end up in root"));
        tree.insert((0, "should be added to left child"));
        tree.insert((7, "this will fill right"));
        tree.insert((8, "should cause 6 to bump up to root node"));
        print_tree(&tree.root, 0);
    }

    #[test]
    fn add_test_4() {
        let mut tree = BTree::new(5);
        tree.insert((1, "First item"));
        tree.insert((2, "Second item"));
        tree.insert((3, "This item should end up in the root"));
        tree.insert((4, "First item, in right child"));
        tree.insert((5, "Second item, in right child"));
        assert!(!tree.root.is_none());

        // add more nodes
        tree.insert((6, "should be added to right child, and eventually end up in root"));
        tree.insert((0, "should be added to left child"));
        tree.insert((7, "this will fill right"));
        tree.insert((8, "should cause 6 to bump up to root node"));
        tree.insert((-1, "goes into left child"));
        tree.insert((-2, "should bump zero up to the root"));
        tree.insert((9, "will eventually end up in root"));
        tree.insert((10, "right child"));
        tree.insert((11, "should bump 9 up to the root"));
        tree.insert((12, "right child"));
        tree.insert((13, "right child again"));
        tree.insert((14, "should create a third tier of the tree"));
        
        print_tree(&tree.root, 0);
    }
}
