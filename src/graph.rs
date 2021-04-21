struct Node<T> {
    data: T,
    edges: Vec<Box<Node<T>>>
}

impl<T> Node<T> {
    fn new(data: T) -> Self{
        Node {
            data,
            edges: Vec::new()
        }
    }
}

struct Graph<T> {
    nodes: Vec<Node<T>>
}

impl<T> Graph<T> {
    fn new() -> Self {
        Graph {
            nodes: Vec::new()
        }
    }

    fn add_node(&self, val: T) {
        
    }
}