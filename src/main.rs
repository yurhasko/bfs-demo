use bfs_graph::Graph;

fn main() {
    let mut network = Graph::new();

    network.add_edge("RouterA", "Switch1");
    network.add_edge("RouterA", "RouterB");
    network.add_edge("RouterB", "Switch2");
    network.add_edge("RouterB", "RouterC");
    network.add_edge("RouterC", "Server1");
    network.add_edge("RouterC", "Server2");
    network.add_edge("Switch1", "PC1");
    network.add_edge("Switch1", "PC2");
    network.add_edge("Switch2", "PC3");

    let source = "PC1";
    let destination = "Server2";

    match network.bfs_shortest_path(source, destination) {
        Some(path) => println!("Shortest path: {}", path.join(" -> ")),
        None => println!("No path found from {} to {}", source, destination),
    }
}