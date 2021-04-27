struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>
}

pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
    size: usize
}

impl<T> Stack<T> {
    /// Constructs a new empty stack
    pub fn new() -> Self {
        Stack {
            top: None,
            size: 0
        }
    }

    /// checks to see if the stack is empty or not
    pub fn is_empty(&self) -> bool {
        match self.top {
            Some(_) => false,
            None => {
                assert_eq!(self.size, 0);
                true
            }
        }
    }

    /// Add items to the top of the stack
    pub fn push(&mut self, item: T) {
        let prev_top = self.top.take();
        self.top = Some(Box::new(Node { data: item, next: prev_top}));
        self.size += 1;
    }

    /// Remove items from the top of the stack
    /// Returns None if the stack is empty
    pub fn pop(&mut self) -> Option<T> {
        let ret = self.top.take();
        if let Some(node) = ret {
            self.top = node.next;
            self.size -= 1;
            Some(node.data)
        } else {
            None
        }
    }

    /// returns number of items in the stack
    pub fn depth(&self) -> usize {
        self.size
    }

    /// returns a reference to the top item without removing it
    /// returns None if the stack is empty
    pub fn peek(&self) -> Option<&T> {
        match &self.top {
            Some(node) => Some(&node.data),
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> Stack<i32> {
        Stack::new()
    }

    #[test]
    fn test_is_empty_true() {
        let stack = init();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_push_no_panic() {
        let mut stack = init();
        stack.push(3);
    }

    #[test]
    fn test_is_empty_false() {
        let mut stack = init();
        stack.push(3);
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_pop_empty() {
        let mut stack = init();
        assert!(stack.is_empty());
        if let Some(_) = stack.pop() { 
            panic!("shouldn't have been able to pop anything"); 
        }
        assert!(stack.is_empty());
    }

    #[test]
    fn test_pop_one_item() {
        let mut stack = init();
        stack.push(3);
        assert!(!stack.is_empty());
        match stack.pop() {
            Some(val) => assert_eq!(val, 3),
            None => panic!("should've popped something")
        }
        assert!(stack.is_empty());
    }

    #[test]
    fn test_pop_multiple_items() {
        let mut stack = init();
        let values = vec![3, 6, 9, 12, 22];
        for i in values.iter() {
            stack.push(i.clone());
        }
        assert!(!stack.is_empty());
        for i in values.iter().rev() {
            match stack.pop() {
                Some(val) => assert_eq!(i, &val),
                None => panic!("values not returned in correct order")
            }
        }
    }

    #[test]
    fn test_depth_empty() {
        let stack = init();
        assert_eq!(0, stack.depth());
    }

    #[test]
    fn test_depth_one() {
        let mut stack = init();
        stack.push(3);
        assert_eq!(1, stack.depth());
    }

    #[test]
    fn test_depth_many() {
        let mut stack = init();
        let nums = vec![3,6,9,12,15];
        let remove = 2;
        for i in nums.iter() {
            stack.push(*i);
        }
        assert_eq!(nums.len(), stack.depth());
        let mut i = 0;
        while i < remove {
            stack.pop();
            i+= 1;
        }
        assert_eq!(nums.len()-remove, stack.depth());
    }

    #[test]
    fn test_peek_empty() {
        let stack = init();
        if let Some(_) = stack.peek() {
            panic!("shouldn't be anything to peek");
        }
    }

    #[test]
    fn test_peek_items() {
        let mut stack = init();
        stack.push(5);
        let size = stack.depth();
        let mut reference = 0;
        if let Some(val) = stack.peek() {
            assert_eq!(*val, 5);
            assert_eq!(stack.depth(), size);
            reference = *val;
        } else {
            panic!("should've had something to peek")
        }
        if let Some(returned) = stack.pop() {
            assert_eq!(returned, reference);
        }
    }
}