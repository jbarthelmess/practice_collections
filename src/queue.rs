struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

pub struct Queue<T> {
    front: Option<Box<Node<T>>>,
    size: usize
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None
        }
    }

    fn append(&mut self, data: T) {
        if let Some(val) = &mut self.next {
            val.append(data);
        } else {
            self.next = Some(Box::new(Node::new(data)));
        }
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            front: None,
            size: 0
        }
    }

    pub fn push(&mut self, data: T) {
        if let Some(val) = &mut self.front {
            val.append(data);
        } else {
            self.front = Some(Box::new(Node::new(data)));
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(val) = self.front.take() {
            self.front = val.next;
            self.size -= 1;
            Some(val.data)
        } else {
            None
        }
    }
}