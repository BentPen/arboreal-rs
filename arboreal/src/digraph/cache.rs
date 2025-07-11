
use std::fmt::Debug;

use fixed_deque::Deque;

use super::{DirEdge, GraphChange, Nodal, DiGraph};

const UNDO_HISTORY_LIMIT: usize = 100;

#[derive(PartialEq)]
pub struct HistoryDeque<N, E> (Deque<GraphChange<N, E>>);

impl<N, E> HistoryDeque<N, E> {
    pub fn new(limit: usize) -> Self {
        Self(Deque::new(limit))
    }
}

impl<N, E> Default for HistoryDeque<N, E> {
    fn default() -> Self {
        Self::new(UNDO_HISTORY_LIMIT)
    }
}

impl<N, E> Debug for HistoryDeque<N, E> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

/// Trait for implementing optional cache usage.
/// 
/// Trait is required for DiGraph, but default implementation does nothing
/// if mut_history() returns None.
pub(super) trait ChangeCache<N: Nodal, E: DirEdge> {

    /// Points to deque of GraphChange variants, for ease of undoing operations
    /// 
    /// To disable use of this deque, have this return None
    fn mut_history(&mut self) -> Option<&mut HistoryDeque<N, E>>;

    /// Erases all entries but keeps the same max length
    fn clear_history(&mut self) {
        if let Some(hist_deq) = self.mut_history() {
            hist_deq.0.clear();
        }
    }

    /// Adds change variant to HistoryDeque; returns oldest item in HistoryDeque if at capacity
    fn register_change(&mut self, change: GraphChange<N, E>) -> Option<GraphChange<N, E>> {
        if let Some(hist_deq) = self.mut_history() {
            let first_in = hist_deq.0.push_back(change);
            return first_in;
        }
        None
    }

    /// Removes and returns most recent change variant, or None if HistoryDeque is empty
    fn pop_change(&mut self) -> Option<GraphChange<N, E>> {
        if let Some(hist_deq) = self.mut_history() {
            return hist_deq.0.pop_back();
        }
        None
    }

}

impl<N: Nodal, E: DirEdge> ChangeCache<N, E> for DiGraph<N, E> {
    fn mut_history(&mut self) -> Option<&mut HistoryDeque<N, E>> {
        Some(&mut self.undo_history)
    }
}

impl<N: Nodal, E: DirEdge> DiGraph<N, E> {
    pub fn undo(&mut self) -> Result<(), &'static str> {
        if let Some(change_to_reverse) = self.pop_change() {
            match change_to_reverse {
                GraphChange::AddNode(node) => {
                    self.remove_node_unregistered(node.node_id());
                },
                GraphChange::RemoveNode(node, edges) => {
                    self.insert_node_unregistered(node);
                    for edge in edges.into_iter() {
                        self.insert_edge_unregistered(edge);
                    }
                },
                GraphChange::AddEdge(edge) => {
                    let edge_index = self.edge_index(edge.start_id(), edge.end_id()).unwrap();
                    self.remove_edge_unregistered(edge_index);
                },
                GraphChange::AddEdgeWith(edge, new_start, new_end) => {
                    if let Some(node_id) = new_start {
                        self.remove_node_unregistered(node_id);
                    }
                    if let Some(node_id) = new_end {
                        self.remove_node_unregistered(node_id);
                    }
                    if let Some(edge_index) = self.edge_index(edge.start_id(), edge.end_id()) {
                        // This should not trigger if either new_start or new_end is Some(node_id)
                        self.remove_edge_unregistered(edge_index);
                    }
                },
                GraphChange::RemoveEdge(edge) => {
                    self.insert_edge_unregistered(edge);
                },
                GraphChange::InsertNodeAlongEdge(node, edge) => {
                    self.remove_node_unregistered(node.node_id());
                    self.insert_edge_unregistered(edge);
                },
                GraphChange::Failure(msg) => return Err(msg), // should be impossible with how mut_history is set up.
            }
        }
        Ok(())
    }

}
