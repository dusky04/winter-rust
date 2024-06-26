use std::collections::HashSet;

use crate::scalar::Scalar;

pub fn topological_sort(
    vertex: &Scalar,
    visited: &mut HashSet<Scalar>,
    sorted_nodes: &mut Vec<Scalar>,
) {
    // Topological order of all the children in the graph
    if !visited.contains(vertex) {
        visited.insert(vertex.clone());
        for child in vertex._children.iter() {
            topological_sort(vertex, visited, sorted_nodes);
        }
        sorted_nodes.push(vertex.clone());
    }
}
