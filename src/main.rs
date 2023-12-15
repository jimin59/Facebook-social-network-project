use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod data;
mod graph;

use crate::data::random_nodes;
use crate::graph::{six_degrees_of_separation, average_and_std_dev_of_highest_degree, nodes_separated_by_degree};

fn load_facebook_data(file_name: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    // Construct the full file path
    let file_path = Path::new("src").join(file_name);
    let file = File::open(&file_path)?;

    let reader = BufReader::new(file);

    let mut graph = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let line_parts: Vec<&str> = line.split_whitespace().collect();

        if line_parts.len() != 2 {
            eprintln!("Invalid data format: {}", line);
            continue;
        }

        let node = line_parts[0].to_string();
        let neighbor = line_parts[1].to_string();

        graph.entry(node).or_insert_with(Vec::new).push(neighbor);
    }

    Ok(graph)
}

fn main() -> Result<(), Box<dyn Error>> {
    // File name of your local dataset in the "src" directory
    let file_name = "facebook_combined.txt";

    // Load and process the data
    let graph = load_facebook_data(file_name)?;

    // Randomly select 10,000 nodes
    let selected_nodes: HashSet<_> = random_nodes(&graph, 10_000).into_iter().map(|s| s.to_string()).collect();

    // Six degrees of separation for selected nodes
    let average_distance = six_degrees_of_separation(&graph, &selected_nodes);
    println!("Average distance for selected nodes: {}", average_distance);

    // Average and standard deviation of the highest connection (degree) in the graph
    let (average_degree, std_dev_degree) = average_and_std_dev_of_highest_degree(&graph);
    println!("Average Degree: {}", average_degree);
    println!("Standard Deviation of Degree: {}", std_dev_degree);

    // Count nodes separated by one, two, three, etc. for each degree
    let separation_counts = nodes_separated_by_degree(&graph);
    println!("Nodes Separated by Degree:");
    for (degree, counts) in separation_counts.iter() {
        println!("Degree {}: {:?}", degree, counts);
    }

    Ok(())
}

