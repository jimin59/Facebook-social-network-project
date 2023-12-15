use statrs::statistics::Statistics;
use std::collections::HashMap;
use std::collections::HashSet;

// Function to perform a breadth-first search (BFS) on the graph
pub fn bfs<'a>(graph: &'a HashMap<String, Vec<String>>, start_node: &'a str, selected_nodes: &'a HashSet<String>) -> usize {
    let mut visited = HashSet::new(); // Set to keep track of visited nodes
    let mut queue = vec![start_node]; // Queue initialized with the start node

    let mut distance = 0; // Variable to store the total distance

    // Loop until the queue is empty
    while let Some(current_node) = queue.pop() {
        // If the current node has been visited before, skip the rest of the loop
        if visited.contains(current_node) {
            continue;
        }

        // Mark the current node as visited
        visited.insert(current_node);

        // If the current node is in the set of selected nodes, increment the distance
        if selected_nodes.contains(current_node) {
            distance += 1;
        }

        // If the current node has neighbors, add them to the queue
        if let Some(neighbors) = graph.get(current_node) {
            for neighbor in neighbors {
                queue.push(neighbor);
            }
        }
    }

    // Return the total distance
    distance
}

// Function to calculate the average shortest path length in the graph
pub fn six_degrees_of_separation(graph: &HashMap<String, Vec<String>>, selected_nodes: &HashSet<String>) -> f64 {
    // Calculate the total distance by performing a BFS for each selected node
    let total_distance: usize = selected_nodes.iter().map(|node| bfs(graph, node, selected_nodes)).sum();
    // Calculate and return the average distance
    total_distance as f64 / selected_nodes.len() as f64
}

// Function to calculate the average degree and standard deviation of degree in the graph
pub fn average_and_std_dev_of_highest_degree(graph: &HashMap<String, Vec<String>>) -> (f64, f64) {
    // Create a vector of the degrees of all nodes
    let degrees: Vec<f64> = graph.values().map(|neighbors| neighbors.len() as f64).collect();
    // Calculate the average degree
    let average_degree = Statistics::mean(&degrees);
    // Calculate the standard deviation of degree
    let std_dev_degree = Statistics::std_dev(&degrees);

    // Return the average degree and standard deviation of degree
    (average_degree, std_dev_degree)
}

// Function to count the number of nodes separated by each degree
pub fn nodes_separated_by_degree(graph: &HashMap<String, Vec<String>>) -> HashMap<usize, Vec<usize>> {
    let mut separation_counts: HashMap<usize, Vec<usize>> = HashMap::new();

    // Loop over all nodes in the graph
    for neighbors in graph.values() {
        // Get the degree of the current node
        let degree = neighbors.len();
        // Get the vector of counts for the current degree, or create a new one if it doesn't exist
        let separation_count = separation_counts.entry(degree).or_insert_with(|| vec![0; degree]);

        // Increment the count for each degree up to the current degree
        for count in 1..=degree {
            *separation_count.get_mut(count - 1).unwrap() += 1;
        }
    }

    // Return the counts
    separation_counts
}

