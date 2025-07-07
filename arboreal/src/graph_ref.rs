
use crate::graph_components::*;

// Type aliases for frequently used collections
type NV<I, TN> = Vec<Node<I, TN>>;
type EV<I, TE> = Vec<Edge<I, TE>>;

pub fn node_index<I, TN: Clone>(nodes: &NV<I, TN>, query_id: I) -> Option<usize> 
where 
    I: Copy + PartialEq + Eq
{
    // This method may be overloaded to utilize cached values or a hash map
    for (index, node) in nodes.iter().enumerate() {
        if node.0 == query_id {
            return Some(index);
        }
    }
    None
}

pub fn edge_index<I, TE: Clone>(edges: &EV<I, TE>, id_in: I, id_out: I) -> Option<usize> 
where 
    I: Copy + PartialEq + Eq
{
    for (index, edge) in edges.iter().enumerate() {
        if edge.0 == id_in && edge.1 == id_out {
            return Some(index);
        }
    }
    None
}

pub fn check_add_node<I, TN: Clone, TE: Clone>(nodes: &NV<I, TN>, new_node: Node<I, TN>) -> GraphChange<I, TN, TE> 
where 
    I: Copy + PartialEq + Eq
{
    if let Some(_index) = node_index(nodes, new_node.0) {
        return GraphChange::Failure("Node with this id already exists.");
    }
    GraphChange::AddNode(new_node)
}

/// Determines in- and out-degree of `id` in `edges`.
/// If no edge is found with a terminal at this `id`, `(0, 0)` is returned.
/// 
/// Note that this does not take any reference to a vector of `Node`, 
/// so the caller must take care to ensure the input `id` is valid
/// in whatever collection of `Node` is being used.
pub fn check_node_degrees<I, TE: Clone>(edges: &EV<I, TE>, id: I) -> (usize, usize) 
where 
    I: Copy + PartialEq + Eq
{
    let mut degs = (0, 0);
    for edge in edges.iter() {
        if edge.1 == id {
            degs.0 += 1;
        }
        if edge.0 == id {
            degs.1 += 1;
        }
    }
    degs
}

pub fn check_remove_node<I, TN: Clone, TE: Clone>(nodes: &NV<I, TN>, edges: &EV<I, TE>, id: I) -> GraphChange<I, TN, TE> 
where 
    I: Copy + PartialEq + Eq
{
    let index = node_index(nodes, id);
    if index.is_none() {
        return GraphChange::Failure("Node with this id not found.");
    }
    let index = index.unwrap();
    let node_to_discard = nodes[index].clone();
    let mut edges_to_drop = Vec::with_capacity(nodes.len());
    let keep_rule = |e: &Edge<I, TE>| !(e.0 == id || e.1 == id);
    for edge in edges.iter() {
        if !keep_rule(edge) {
            edges_to_drop.push(edge.clone());
        }
    }
    GraphChange::RemoveNode(node_to_discard, edges_to_drop)
}

pub fn check_add_edge<I, TN: Clone, TE: Clone>(nodes: &NV<I, TN>, edges: &EV<I, TE>, new_edge: Edge<I, TE>) -> GraphChange<I, TN, TE> 
where 
    I: Copy + PartialEq + Eq
{
    if let Some(_index) = edge_index(edges, new_edge.0, new_edge.1) {
        return GraphChange::Failure("Edge with these terminals already exists.");
    }
    if let Some(_left) = node_index(nodes, new_edge.0)
        && let Some(_right) = node_index(nodes, new_edge.1) 
    {
        return GraphChange::AddEdge(new_edge)
    }
    GraphChange::Failure("Terminals not found in graph.")
}

pub fn check_add_edge_with_nodes<I, TN: Clone, TE: Clone>(nodes: &NV<I, TN>, edges: &EV<I, TE>, id_in: I, id_out: I) -> GraphChange<I, TN, TE> 
where 
    I: Copy + PartialEq + Eq
{
    if let Some(_index) = edge_index(edges, id_in, id_out) {
        return GraphChange::Failure("Edge with these terminals already exists.");
    }
    let new_in = match node_index(nodes, id_in) {
        Some(_index) => None, 
        None => Some(id_in)
    };
    let new_out = match node_index(nodes, id_out) {
        Some(_index) => None, 
        None => Some(id_out)
    };
    GraphChange::AddEdgeWith(Edge::bare(id_in, id_out), new_in, new_out)
}

pub fn check_remove_edge<I, TN: Clone, TE: Clone>(edges: &EV<I, TE>, id_in: I, id_out: I) -> GraphChange<I, TN, TE> 
where 
    I: Copy + PartialEq + Eq
{
    if let Some(index) = edge_index(edges, id_in, id_out) {
        let edge_to_drop = edges[index].clone();
        return GraphChange::RemoveEdge(edge_to_drop);
    }
    GraphChange::Failure("Edge not found in graph.")
}
