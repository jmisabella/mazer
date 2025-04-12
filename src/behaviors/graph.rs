use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

/// Perform a breadth-first search starting from `start`,
/// returning a mapping of each reachable node to its distance from `start`.
/// 
/// # Arguments
/// 
/// * `start` - The node at which to begin the search.
/// * `neighbors` - A closure that, given a node, returns a `Vec` of its neighboring nodes.
/// 
/// # Returns
/// 
/// A `HashMap` mapping each node to its distance from the starting node.
pub fn bfs_distances<Node, F>(start: Node, neighbors: F) -> HashMap<Node, u32>
where
    Node: Eq + Hash + Copy,
    F: Fn(Node) -> Vec<Node>,
{
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    distances.insert(start, 0);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        let current_distance = distances[&current];
        for neighbor in neighbors(current) {
            if !distances.contains_key(&neighbor) {
                distances.insert(neighbor, current_distance + 1);
                queue.push_back(neighbor);
            }
        }
    }
    distances
}

/// Reconstructs a path from `start` to `goal` given a precomputed `distances` map.
///
/// # Arguments
///
/// * `start` - The starting node.
/// * `goal` - The destination node.
/// * `distances` - A map from nodes to their distance from the start (usually computed by `bfs_distances`).
/// * `neighbors` - A closure that, given a node, returns a `Vec` of its neighboring nodes.
///
/// # Returns
///
/// An `Option<Vec<Node>>` containing the path from start to goal (inclusive) if one exists.
pub fn get_path<Node, F>(
    start: Node,
    goal: Node,
    distances: &HashMap<Node, u32>,
    neighbors: F,
) -> Option<Vec<Node>>
where
    Node: Eq + Hash + Copy,
    F: Fn(Node) -> Vec<Node>,
{
    // If goal wasn't reached, return None.
    if !distances.contains_key(&goal) {
        return None;
    }

    let mut path = Vec::new();
    let mut current = goal;
    path.push(current);

    while current != start {
        let current_distance = distances[&current];
        // Among the neighbors of current, choose one that is one less in distance.
        let prev_opt = neighbors(current)
            .into_iter()
            .find(|&n| distances.get(&n).map_or(false, |&d| d == current_distance - 1));
        if let Some(prev) = prev_opt {
            path.push(prev);
            current = prev;
        } else {
            // If no valid neighbor is found, something is wrong.
            return None;
        }
    }

    path.reverse();
    Some(path)
}

/// Returns all nodes connected (reachable) from `start` using a BFS.
///
/// # Arguments
///
/// * `start` - The starting node.
/// * `neighbors` - A closure that, given a node, returns a `Vec` of its neighboring nodes.
///
/// # Returns
///
/// A `HashSet` containing all nodes reachable from `start`.
pub fn all_connected<Node, F>(start: Node, neighbors: F) -> HashSet<Node>
where
    Node: Eq + Hash + Copy,
    F: Fn(Node) -> Vec<Node>,
{
    let mut connected = HashSet::new();
    let mut queue = VecDeque::new();
    connected.insert(start);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        for neighbor in neighbors(current) {
            if connected.insert(neighbor) {
                queue.push_back(neighbor);
            }
        }
    }
    connected
}
