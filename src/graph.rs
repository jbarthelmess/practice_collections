use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub struct Graph<T: Hash+Eq, W: Ord+Copy> {
    nodes: HashMap<T, Node<T>>,
    edges: HashMap<T, Vec<Edge<T, W>>>
}

pub struct Node<T> {
    id: T
}

pub struct Edge<T, W: Ord+Copy> {
    weight: W,
    node_to: T
}

impl<T: Hash+Eq+Copy, W: Ord+Copy> Graph<T, W> {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: T) {
        if !self.nodes.contains_key(&id) {
            self.nodes.insert(id, Node::new(id));
        }
    }

    pub fn add_edge(&mut self, id_from: T, id_to: T, weight: W) {
        if self.nodes.contains_key(&id_to) && self.nodes.contains_key(&id_from) {
            if !self.edges.contains_key(&id_from) {
                self.edges.insert(id_from, vec![Edge::new(id_to, weight)]);
            } else {
                let edge_vector = self.edges.get_mut(&id_from).unwrap();
                edge_vector.push(Edge::new(id_to, weight));
            }
        }
    }

    pub fn get_edges_from(&self, id: T) -> Option<Vec<(T, W)>> {
        let edges = self.edges.get(&id);
        if let Some(vals) = edges {
            Some(vals.iter().map(|e| (e.node_to, e.weight)).collect())
        } else {
            None
        }
    }
}

impl<T> Node<T> {
    pub fn new(id: T) -> Self {
        Node {
            id
        }
    }
}

impl<T, W: Ord+Copy> Edge<T, W> {
    pub fn new(to: T, weight: W) -> Self {
        Edge {
            weight,
            node_to: to
        }
    }
}
