
use std::collections::HashMap;
use std::fmt::{self, Display};

use crate::graph_base::graph_components::Id;
use crate::graph_base::graph_ref;
use super::{Nodal, DirEdge, DiGraph, HistoryDeque};

const DEFAULT_NODE_PREALLOCATION: usize = 20;
const EXPECTED_EDGES_PER_NODE: usize = 3;

impl<N: Nodal, E: DirEdge> DiGraph<N, E> {

    pub(super) fn edge_index(&self, start_id: Id, end_id: Id) -> Option<usize> {
        for (index, edge) in self.edges.iter().enumerate() {
            if edge.terminal_ids() == (start_id, end_id) {
                return Some(index);
            }
        }
        None
    }

    pub(super) fn insert_node_unregistered(&mut self, node: N) {
        let node_id = node.node_id();
        self.nodes.insert(node_id, node);
        self.neighbors_before.insert(node_id, Vec::with_capacity(5));
        self.neighbors_after.insert(node_id, Vec::with_capacity(5));
    }

    pub(super) fn remove_node_unregistered(&mut self, node_id: Id) -> N {
        for id_before in self.neighbors_before.get(&node_id).unwrap().to_owned() {
            let edge_index = self.edge_index(id_before, node_id).unwrap();
            self.remove_edge_unregistered(edge_index);
        }
        for id_after in self.neighbors_after.get(&node_id).unwrap().to_owned() {
            let edge_index = self.edge_index(node_id, id_after).unwrap();
            self.remove_edge_unregistered(edge_index);
        }
        self.nodes
            .remove(&node_id)
            .unwrap()
    }

    pub(super) fn insert_edge_unregistered(&mut self, edge: E) {
        let (start_id, end_id) = edge.terminal_ids();
        self.edges.push(edge);
        // Register end node's id as start node's after-neighbor
        self.neighbors_after
            .get_mut(&start_id)
            .unwrap()
            .push(end_id);
        // Register start node's id as end node's before-neighbor
        self.neighbors_before
            .get_mut(&end_id)
            .unwrap()
            .push(start_id);
    }

    pub(super) fn remove_edge_unregistered(&mut self, edge_index: usize) {
        let dropped_edge = self.edges.swap_remove(edge_index);
        let (start_id, end_id) = dropped_edge.terminal_ids();
        self.neighbors_before
            .get_mut(&end_id)
            .unwrap()
            .retain(|&x| x != start_id);
        self.neighbors_after
            .get_mut(&start_id)
            .unwrap()
            .retain(|&x| x != end_id);
    }

    /// Returns vec of `node_id` for which `in_degree(node_id) == Some(0)`
    pub(super) fn source_node_ids(&self) -> Vec<Id> {
        let mut ids: Vec<Id> = self.all_node_ids();
        ids.retain(|&id| self.in_degree(id) == Some(0));
        ids.shrink_to_fit();
        ids
    }

    /// Returns vec of `node_id` for which `out_degree(node_id) == Some(0)`
    pub(super) fn sink_node_ids(&self) -> Vec<Id> {
        let mut ids: Vec<Id> = self.all_node_ids();
        ids.retain(|&id| self.out_degree(id) == Some(0));
        ids.shrink_to_fit();
        ids
    }

    // pub(super) fn path_from_source(&self, id: Id) -> Result<Vec<Id>, &'static str> {
    //     let source = self.get_source()?;
    //     todo!()
    // }
    // pub(super) fn path_to_sink(&self, id: Id) -> Result<Vec<Id>, &'static str> {
    //     todo!()
    // }

    pub(super) fn is_connected(&self) -> bool {
        let mut source_ids = self.source_node_ids();
        if source_ids.len() != 1 {
            return false;
        }
        let starting_point = source_ids.pop().unwrap();
        self.nodes_unreachable_from(starting_point).is_empty()
    }
    pub(super) fn is_terminable(&self) -> bool {
        todo!()
    }
    pub(super) fn is_valid(&self) -> bool {
        self.is_connected() && self.is_terminable()
    }
}

impl<N: Nodal, E: DirEdge> Default for DiGraph<N, E> {
    fn default() -> Self {
        let n = DEFAULT_NODE_PREALLOCATION;
        let e = EXPECTED_EDGES_PER_NODE * n;
        let name = None;
        let nodes = HashMap::with_capacity(n);
        let edges = Vec::with_capacity(e);
        let neighbors_before = HashMap::with_capacity(n);
        let neighbors_after = HashMap::with_capacity(n);
        let undo_history = HistoryDeque::default();
        Self { name, nodes, edges, neighbors_before, neighbors_after, undo_history }
    }
}

impl<N: Nodal, E: DirEdge> Display for DiGraph<N, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match &self.name {
            Some(name) => format!("DiGraph ('{}')", name),
            None => "DiGraph".to_string()
        };
        let edges_per_line: u8 = 4;
        write!(f, "{name}\n")?;
        write!(f, "\tNode Ids: {:?}\n", self.all_node_ids())?;
        if self.edges.len() > 0 {
            let edge_vec = self.all_edge_pairs();
            let mut reset_count = 0;
            for edge_pair in edge_vec.into_iter() {
                if reset_count == 0 {
                    write!(f, "\t")?;
                } else {
                    write!(f, ",  ")?;
                }
                write!(f, "{}->{}", edge_pair.0, edge_pair.1)?;
                reset_count += 1;
                if reset_count == edges_per_line {
                    reset_count = 0;
                    write!(f, "\n")?;
                }
            }
        } else {
            write!(f, "\t(No edges)")?;
        }
        Ok(())
    }
}
