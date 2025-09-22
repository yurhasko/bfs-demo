use std::collections::{HashMap, HashSet, VecDeque};

/// A simple undirected graph with adjacency lists.
#[derive(Debug, Default)]
pub struct Graph {
    adjacency_list: HashMap<String, Vec<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    /// Add an undirected edge between two nodes.
    pub fn add_edge(&mut self, node1: &str, node2: &str) {
        self.adjacency_list
            .entry(node1.to_string())
            .or_default()
            .push(node2.to_string());
        self.adjacency_list
            .entry(node2.to_string())
            .or_default()
            .push(node1.to_string());
    }

    /// Find the shortest path between `source` and `destination` using BFS.
    /// Returns `Some(Vec<String>)` if a path is found, or `None` if there is no connection.
    pub fn bfs_shortest_path(&self, source: &str, destination: &str) -> Option<Vec<String>> {
        if !self.adjacency_list.contains_key(source) || !self.adjacency_list.contains_key(destination) {
            return None;
        }

        let mut queue: VecDeque<Vec<String>> = VecDeque::new();
        let mut visited: HashSet<String> = HashSet::new();

        queue.push_back(vec![source.to_string()]);
        visited.insert(source.to_string());

        while let Some(path) = queue.pop_front() {
            let last_node = path.last().unwrap();

            if last_node == destination {
                return Some(path);
            }

            if let Some(neighbors) = self.adjacency_list.get(last_node) {
                for neighbor in neighbors {
                    if visited.insert(neighbor.clone()) {
                        let mut new_path = path.clone();
                        new_path.push(neighbor.clone());
                        queue.push_back(new_path);
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph() -> Graph {
        let mut graph = Graph::new();
        graph.add_edge("RouterA", "Switch1");
        graph.add_edge("RouterA", "RouterB");
        graph.add_edge("RouterB", "Switch2");
        graph.add_edge("RouterB", "RouterC");
        graph.add_edge("RouterC", "Server1");
        graph.add_edge("RouterC", "Server2");
        graph.add_edge("Switch1", "PC1");
        graph.add_edge("Switch1", "PC2");
        graph.add_edge("Switch2", "PC3");
        graph
    }

    #[test]
    fn test_shortest_path_exists() {
        let graph = sample_graph();
        let path = graph.bfs_shortest_path("PC1", "Server2").unwrap();
        assert_eq!(
            path,
            vec!["PC1", "Switch1", "RouterA", "RouterB", "RouterC", "Server2"]
        );
    }

    #[test]
    fn test_no_path() {
        let mut graph = sample_graph();
        // An isolated node with no connections
        graph.add_edge("Isolated", "Nowhere");
        let path = graph.bfs_shortest_path("PC1", "Isolated");
        assert!(path.is_none());
    }

    #[test]
    fn test_source_or_destination_not_in_graph() {
        let graph = sample_graph();
        assert!(graph.bfs_shortest_path("Nonexistent", "PC1").is_none());
        assert!(graph.bfs_shortest_path("PC1", "Ghost").is_none());
    }

    #[test]
    fn test_trivial_path() {
        let graph = sample_graph();
        let path = graph.bfs_shortest_path("PC1", "PC1").unwrap();
        assert_eq!(path, vec!["PC1"]);
    }
}