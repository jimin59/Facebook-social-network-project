use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, BufRead};
use std::io::Cursor;
use rand::seq::SliceRandom;

use statrs::distribution::DiscreteCDF;

type Node = String;
type Graph = HashMap<Node, Vec<Node>>;

pub fn degree_distribution(graph: &Graph) -> HashMap<usize, usize> {
    let mut degree_counts = HashMap::new();

    for (_, friends) in graph.iter() {
        let degree = friends.len();
        *degree_counts.entry(degree).or_insert(0) += 1;
    }

    degree_counts
}

pub fn friends_of_friends_overlap(graph: &Graph, selected_nodes: &HashSet<&Node>) -> usize {
    let mut friends_of_friends = HashSet::new();

    for &node in selected_nodes.iter() {
        if let Some(friends) = graph.get(node) {
            for friend in friends.iter() {
                if let Some(fof) = graph.get(friend) {
                    friends_of_friends.extend(fof);
                }
            }
        }
    }

    let overlapping_friends: HashSet<_> = friends_of_friends.intersection(selected_nodes).collect();
    overlapping_friends.len()
}

// Placeholder for the missing function
fn load_facebook_data(url: &str) -> Result<Graph, Box<dyn Error>> {
    // TODO: Implement this function
    Ok(Graph::new())
}

// Placeholder for the missing function
fn random_nodes(graph: &Graph, num: usize) -> Vec<&Node> {
    // TODO: Implement this function
    vec![]
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let graph = load_facebook_data("https://law.di.unimi.it/webdata/fb-current/")?;
    let selected_nodes: HashSet<_> = random_nodes(&graph, 10_000).into_iter().collect();

    // Degree distribution
    let degree_counts = degree_distribution(&graph);
    println!("Degree Distribution:");
    for (degree, count) in degree_counts.iter() {
        println!("Degree {}: Count {}", degree, count);
    }

    // Friends of friends overlap
    let overlap_count = friends_of_friends_overlap(&graph, &selected_nodes);
    println!("Friends of Friends Overlap: {}", overlap_count);

    Ok(())
}

