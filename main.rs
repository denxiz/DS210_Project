use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

// Define a struct to represent a graph
pub struct Graph {
    edges: HashMap<u32, Vec<u32>>,
}

impl Graph {
    // Create a new instance of the graph
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    // Add an edge to the graph
    pub fn add_edge(&mut self, from: u32, to: u32) {
        self.edges.entry(from).or_insert(Vec::new()).push(to);
    }

    // Perform Breadth-First Search to calculate shortest paths from a source node
    fn bfs(&self, source: u32) -> HashMap<u32, usize> {
        let mut visited: HashSet<u32> = HashSet::new();
        let mut distances: HashMap<u32, usize> = HashMap::new();
        let mut queue: Vec<(u32, usize)> = Vec::new();

        visited.insert(source);
        queue.push((source, 0));

        while let Some((node, distance)) = queue.pop() {
            distances.insert(node, distance);
            if let Some(neighbors) = self.edges.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push((neighbor, distance + 1));
                    }
                }
            }
        }

        distances
    }

    // Calculate the average shortest path length from a source node to all other nodes
    pub fn average_shortest_path_length(&self, source: u32) -> f64 {
        let distances = self.bfs(source);
        let num_nodes = self.edges.len() as f64;

        let total_distance: usize = distances.values().sum();
        total_distance as f64 / num_nodes
    }

    // Calculate the standard deviation of the average shortest path lengths from a source node to all other nodes
    pub fn standard_deviation(&self, source: u32) -> f64 {
        let distances = self.bfs(source);
        let num_nodes = self.edges.len() as f64;

        let average_shortest_path = self.average_shortest_path_length(source);

        let sum_of_squared_differences = distances.values().fold(0.0, |acc, &distance| {
            acc + (distance as f64 - average_shortest_path).powi(2)
        });

        (sum_of_squared_differences / num_nodes).sqrt()
    }

    // Calculate the maximum shortest path length from a source node to all other nodes
    pub fn max_shortest_path_length(&self, source: u32) -> usize {
        let distances = self.bfs(source);
        *distances.values().max().unwrap_or(&usize::MAX)
    }

    // Calculate the minimum shortest path length from a source node to all other nodes
    pub fn min_shortest_path_length(&self, source: u32) -> usize {
        let distances = self.bfs(source);
        *distances.values().min().unwrap_or(&usize::MAX)
    }

    // Calculate the median shortest path length from a source node to all other nodes
    pub fn median_shortest_path_length(&self, source: u32) -> usize {
        let mut distances: Vec<usize> = self.bfs(source).values().cloned().collect();
        distances.sort();
        let n = distances.len();
        if n % 2 == 0 {
            (distances[n / 2 - 1] + distances[n / 2]) / 2
        } else {
            distances[n / 2]
        }
    }

    // Calculate the distribution of shortest path lengths from a source node to all other nodes
    pub fn shortest_path_length_distribution(&self, source: u32) -> HashMap<usize, usize> {
        let mut distribution: HashMap<usize, usize> = HashMap::new();
        let distances = self.bfs(source);

        for &distance in distances.values() {
            *distribution.entry(distance).or_insert(0) += 1;
        }

        distribution
    }
}

fn main() {
    let mut graph = Graph::new();

    // Open the dataset file
    let file = File::open("Amazon0302.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // Parse the dataset and construct the graph
    for line in reader.lines().skip(4) {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.trim().split('\t').collect();
        let from_node: u32 = parts[0].parse().expect("Failed to parse node ID");
        let to_node: u32 = parts[1].parse().expect("Failed to parse node ID");
        graph.add_edge(from_node, to_node);
    }

    // Select a source node for calculating shortest paths
    let source_node = 0;

    // Calculate average shortest path length
    let average_shortest_path = graph.average_shortest_path_length(source_node);
    println!("Average Shortest Path Length from Node {}: {:.2}", source_node, average_shortest_path);

    // Calculate standard deviation of average shortest path lengths
    let standard_deviation = graph.standard_deviation(source_node);
    println!("Standard Deviation of Average Shortest Path Lengths from Node {}: {:.2}", source_node, standard_deviation);

    // Calculate maximum shortest path length
    let max_shortest_path = graph.max_shortest_path_length(source_node);
    println!("Maximum Shortest Path Length from Node {}: {}", source_node, max_shortest_path);

    // Calculate minimum shortest path length
    let min_shortest_path = graph.min_shortest_path_length(source_node);
    println!("Minimum Shortest Path Length from Node {}: {}", source_node, min_shortest_path);

    // Calculate median shortest path length
    let median_shortest_path = graph.median_shortest_path_length(source_node);
    println!("Median Shortest Path Length from Node {}: {}", source_node, median_shortest_path);

    // Calculate shortest path length distribution
    let shortest_path_distribution = graph.shortest_path_length_distribution(source_node);

    // Print the top 10 shortest path lengths and their counts
    println!("Top 10 Shortest Path Lengths and Their Counts from Node {}:", source_node);
    let mut sorted_keys: Vec<usize> = shortest_path_distribution.keys().copied().collect();
    sorted_keys.sort();
    for distance in sorted_keys.iter().take(10) {
        let count = shortest_path_distribution.get(distance).unwrap_or(&0);
        println!("Distance {}: {}", distance, count);
    }
}
