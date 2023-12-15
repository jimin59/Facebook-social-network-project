use rand::distributions::{Distribution, WeightedIndex};
use std::collections::{HashMap, HashSet};

// Function to select a random set of nodes from the graph
pub fn random_nodes<'a>(graph: &'a HashMap<String, Vec<String>>, num_nodes: usize) -> HashSet<&'a String> {
    // Collect all the nodes in the graph into a vector
    let all_nodes: Vec<_> = graph.keys().collect();
    // Create a vector of weights for the weighted random selection,
    // where the weight of each node is its degree (number of neighbors)
    let weights: Vec<_> = graph.values().map(|neighbors| neighbors.len()).collect();

    // Create a random number generator
    let mut rng = rand::thread_rng();
    // Create a weighted index distribution for the weighted random selection
    let dist = WeightedIndex::new(&weights).unwrap();

    // Create a set to store the selected nodes
    let mut selected_nodes = HashSet::new();
    // Keep selecting nodes at random until we have the desired number of nodes
    while selected_nodes.len() < num_nodes {
        // Select a node at random, according to the weighted index distribution
        let node = all_nodes[dist.sample(&mut rng)];
        // Add the selected node to the set of selected nodes
        selected_nodes.insert(node);
    }

    // Return the set of selected nodes
    selected_nodes
}

