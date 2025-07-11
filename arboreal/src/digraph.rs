
mod file;
mod cache;
mod digraph_impl;

pub use file::FileIO;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::graph_base::{graph_components::*, graph_ref};
use cache::{ChangeCache, HistoryDeque};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct DiGraph<N, E> {
    pub name: Option<String>,
    
    nodes: HashMap<Id, N>,
    edges: Vec<E>,

    #[serde(skip)]
    neighbors_before: HashMap<Id, Vec<Id>>,
    #[serde(skip)]
    neighbors_after: HashMap<Id, Vec<Id>>,
    #[serde(skip)]
    undo_history: HistoryDeque<N, E>,
}

impl<N: Nodal, E: DirEdge> DiGraph<N, E> {

    /// `new()` just calls `default()` and may be replaced in the future
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `DirEdge` with bare edges and nodes, using provided Vec of (start, end) id pairs
    pub fn from_terminal_pairs(terminal_pairs: Vec<(Id, Id)>) -> Self {
        let mut instance = Self::new();
        for (start, end) in terminal_pairs {
            instance.insert_edge_with_nodes(start, end).unwrap();
        }
        instance.clear_history();
        instance
    }

    pub fn all_node_ids(&self) -> Vec<Id> {
        let mut node_ids: Vec<Id> = self.nodes
            .keys()
            .cloned()
            .collect();
        node_ids.sort();
        node_ids
    }

    pub fn all_edge_pairs(&self) -> Vec<(Id, Id)> {
        let mut edge_pairs = Vec::with_capacity(self.edges.len());
        for edge in self.edges.iter() {
            edge_pairs.push((edge.start_id(), edge.end_id()));
        }
        // edge_pairs.sort_by(|a,b| a.0 <= b.0 && a.1 <= b.1);
        edge_pairs.sort();
        edge_pairs
    }

    pub fn get_node(&self, node_id: Id) -> Option<&N> {
        self.nodes.get(&node_id)
    }
    pub fn get_node_mut(&mut self, node_id: Id) -> Option<&mut N> {
        self.nodes.get_mut(&node_id)
    }

    pub fn get_edge(&self, start_id: Id, end_id: Id) -> Option<&E> {
        if let Some(index) = self.edge_index(start_id, end_id) {
            return self.edges.get(index);
        }
        None
    }
    pub fn get_edge_mut(&mut self, start_id: Id, end_id: Id) -> Option<&mut E> {
        if let Some(index) = self.edge_index(start_id, end_id) {
            return self.edges.get_mut(index);
        }
        None
    }

    /// Inserts `node` into graph, with no edges.
    /// 
    /// If the node's id is already in use, an error is returned.
    pub fn insert_node(&mut self, node: N) -> Result<(), &'static str> {
        let change = 
            graph_ref::check_add_node::<N, E>(&self.nodes, node);
        let new_node = change.try_get_node()?;
        self.insert_node_unregistered(new_node);
        self.register_change(change);
        Ok(())
    }

    /// Removes and returns node (as Ok(N)) with input id, breaking any edges incident on it.
    /// 
    /// If no node with that id is present in the graph, an error is returned.
    pub fn remove_node(&mut self, node_id: Id) -> Result<N, &'static str> {
        let change =
            graph_ref::check_remove_node::<N, E>(&self.nodes, &self.edges, node_id);
        let out_node_id = change.try_get_node()?.node_id();
        // let out_edges = change.try_get_edge_vec()?;
        let removed_node = self.remove_node_unregistered(out_node_id);
        self.register_change(change);
        Ok(removed_node)
    }

    /// Inserts `edge` into graph.
    /// 
    /// If the edge's terminal nodes are not present in the graph,
    /// or an edge with these same terminals is already present in the graph,
    /// an error is returned.
    pub fn insert_edge(&mut self, edge: E) -> Result<(), &'static str> {
        let change =
            graph_ref::check_add_edge::<N, E>(&self.nodes, &self.edges, edge);
        let new_edge = change.try_get_edge()?;
        self.insert_edge_unregistered(new_edge);
        self.register_change(change);
        Ok(())
    }

    /// Doc TODO
    pub fn remove_edge(&mut self, start_id: Id, end_id: Id) -> Result<(), &'static str> {
        let change = 
            graph_ref::check_remove_edge::<N, E>(&self.edges, start_id, end_id);
        let _out_edge = change.try_get_edge()?;
        let edge_index = self.edge_index(start_id, end_id).unwrap();
        self.remove_edge_unregistered(edge_index);
        self.register_change(change);
        Ok(())
    }

    /// Inserts a bare node with id `new_id` along edge from `id_before` to `id_after`,\
    /// deleting the original edge and creating two new edges.
    /// 
    /// That edge's data is moved to the new edge from `id_before` to `new_id`.
    /// 
    /// If the old edge does not exist, or `new_id` is already in use, an error is returned.
    pub fn insert_node_along(&mut self, new_id: Id, id_before: Id, id_after: Id) -> Result<(), &'static str> {
        let new_node =
            graph_ref::check_add_node::<N, E>(&self.nodes, N::bare(new_id))
            .try_get_node()?;
        let old_edge =
            graph_ref::check_remove_edge::<N, E>(&self.edges, id_before, id_after)
            .try_get_edge()?;
        let edge_index = self.edge_index(id_before, id_after).unwrap();
        let mut edge_before = old_edge.clone();
        edge_before.change_end(new_id);
        let edge_after = E::bare(new_id, id_after);
        self.insert_node_unregistered(new_node.clone());
        self.remove_edge_unregistered(edge_index);
        self.insert_edge_unregistered(edge_before);
        self.insert_edge_unregistered(edge_after);
        self.register_change(GraphChange::InsertNodeAlongEdge(new_node, old_edge));
        Ok(())
    }

    /// Inserts a bare `Edge` with provided terminals, creating bare nodes at those terminals if needed.
    /// 
    /// If an edge with these terminals already exists, an error is returned.
    pub fn insert_edge_with_nodes(&mut self, id_in: Id, id_out: Id) -> Result<(), &'static str> {
        let change =
            graph_ref::check_add_edge_with_nodes::<N, E>(&self.nodes, &self.edges, id_in, id_out);
        let (new_edge, new_in, new_out) = change.try_get_edge_with_nodes()?;
        if let Some(new_id) = new_in {
            self.insert_node_unregistered(N::bare(new_id));
        }
        if let Some(new_id) = new_out {
            self.insert_node_unregistered(N::bare(new_id));
        }
        self.insert_edge_unregistered(new_edge);
        self.register_change(change);
        Ok(())
    }

    /// Returns `Some(n)`, where `n` is the number of edges for which provided `node_id` is the end terminal
    /// 
    /// Or `None` if the provided id is not found among the nodes
    pub fn in_degree(&self, node_id: Id) -> Option<usize> {
        if let Some(ids_before) = self.neighbors_before.get(&node_id) {
            return Some(ids_before.len());
        }
        None
    }

    /// Returns `Some(n)`, where `n` is the number of edges for which provided `node_id` is the start terminal
    /// 
    /// Or `None` if the provided id is not found among the nodes
    pub fn out_degree(&self, node_id: Id) -> Option<usize> {
        if let Some(ids_after) = self.neighbors_after.get(&node_id) {
            return Some(ids_after.len());
        }
        None
    }

    pub fn get_source(&self) -> Result<&N, &'static str> {
        let mut source_ids = self.source_node_ids();
        match source_ids.len() {
            1 => {
                let index = source_ids.pop().unwrap();
                let source_node = self.nodes.get(&index).unwrap();
                Ok(source_node)
            }
            0 => Err("No sources in graph."),
            _ => Err("Multiple sources in graph.")
        }
    }

    pub fn nodes_unreachable_from(&self, starting_point: Id) -> Vec<Id> {
        let mut lost_nodes: Vec<Id> = self.all_node_ids();
        let mut census = Vec::with_capacity(self.nodes.len());
        graph_ref::collect_reachable_neighbors(&mut census, starting_point, &self.neighbors_after);
        lost_nodes.retain(|&id| !census.contains(&id));
        lost_nodes
    }

}
