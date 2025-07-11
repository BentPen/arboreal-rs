
use std::collections::HashMap;

use crate::graph_base::graph_components::*;

type NodeMap<N> = HashMap<Id, N>;

fn edge_index<E: DirEdge>(edges: &Vec<E>, id_in: Id, id_out: Id) -> Option<usize> {
    for (index, edge) in edges.iter().enumerate() {
        if edge.start_id() == id_in && edge.end_id() == id_out {
            return Some(index);
        }
    }
    None
}

fn node_id_present<N: Nodal>(nodes: &NodeMap<N>, id: Id) -> bool {
    nodes.get(&id).is_some()
}

pub fn check_add_node<N: Nodal, E: DirEdge>(nodes: &NodeMap<N>, new_node: N) -> GraphChange<N, E> {
    if node_id_present(nodes, new_node.node_id()) {
        return GraphChange::Failure("Node with this id already exists.");
    }
    GraphChange::AddNode(new_node)
}

/// Determines in- and out-degree of `id` in `edges`.
/// If no edge is found with a terminal at this `id`, `(0, 0)` is returned.
pub fn check_node_degrees<N: Nodal, E: DirEdge>(edges: &Vec<E>, id: Id) -> (usize, usize) {
    let mut degs = (0, 0);
    for edge in edges.iter() {
        if edge.start_id() == id {
            degs.0 += 1;
        }
        if edge.end_id() == id {
            degs.1 += 1;
        }
    }
    degs
}

pub fn check_remove_node<N: Nodal, E: DirEdge>(nodes: &NodeMap<N>, edges: &Vec<E>, id: Id) -> GraphChange<N, E> {
    if !node_id_present(nodes, id) {
        return GraphChange::Failure("Node with this id not found.");
    }
    let node_to_discard = nodes.get(&id)
        .unwrap()
        .clone();
    let mut edges_to_drop = Vec::with_capacity(nodes.len());
    let keep_rule = |e: &E| !(e.start_id() == id || e.end_id() == id);
    for edge in edges.iter() {
        if !keep_rule(edge) {
            edges_to_drop.push(edge.clone());
        }
    }
    GraphChange::RemoveNode(node_to_discard, edges_to_drop)
}

pub fn check_add_edge<N: Nodal, E: DirEdge>(nodes: &NodeMap<N>, edges: &Vec<E>, new_edge: E) -> GraphChange<N, E> {
    let (id_in, id_out) = new_edge.terminal_ids();
    if let Some(_index) = edge_index(edges, id_in, id_out) {
        return GraphChange::Failure("Edge with these terminals already exists.");
    }
    if node_id_present(nodes, id_in) && node_id_present(nodes, id_out)
    {
        return GraphChange::AddEdge(new_edge)
    }
    GraphChange::Failure("Terminals not found in graph.")
}

pub fn check_add_edge_with_nodes<N: Nodal, E: DirEdge>(nodes: &NodeMap<N>, edges: &Vec<E>, id_in: Id, id_out: Id) -> GraphChange<N, E> {
    if let Some(_index) = edge_index(edges, id_in, id_out) {
        return GraphChange::Failure("Edge with these terminals already exists.");
    }
    let new_in = match node_id_present(nodes, id_in) {
        true => None,
        false => Some(id_in)
    };
    let new_out = match node_id_present(nodes, id_out) {
        true => None, 
        false => Some(id_out)
    };
    let proposed_edge = E::bare(id_in, id_out);
    GraphChange::AddEdgeWith(proposed_edge, new_in, new_out)
}

pub fn check_remove_edge<N: Nodal, E: DirEdge>(edges: &Vec<E>, id_in: Id, id_out: Id) -> GraphChange<N, E> {
    if let Some(index) = edge_index(edges, id_in, id_out) {
        let edge_to_drop = edges[index].clone();
        return GraphChange::RemoveEdge(edge_to_drop);
    }
    GraphChange::Failure("Edge not found in graph.")
}

pub fn collect_reachable_neighbors(census: &mut Vec<Id>, starting_point: Id, after_neighbor_map: &HashMap<Id, Vec<Id>>) {
        if census.contains(&starting_point) {
            return
        }
        census.push(starting_point);
        if let Some(cul_da_sac) = after_neighbor_map.get(&starting_point) {
            for neighbor in cul_da_sac.iter() {
                collect_reachable_neighbors(census, *neighbor, after_neighbor_map);
            }
        }
    }
