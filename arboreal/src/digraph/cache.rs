
use std::fmt::Debug;

use fixed_deque::Deque;

use super::{DirEdge, GraphChange, Nodal, DiGraph};

const UNDO_HISTORY_LIMIT: usize = 100;

#[derive(PartialEq)]
pub struct HistoryDeque<N: Nodal, E: DirEdge> (Deque<GraphChange<N, E>>);

impl<N: Nodal, E: DirEdge> HistoryDeque<N, E> {
    pub fn new(limit: usize) -> Self {
        Self(Deque::new(limit))
    }
}

impl<N: Nodal, E: DirEdge> Default for HistoryDeque<N, E> {
    fn default() -> Self {
        Self::new(UNDO_HISTORY_LIMIT)
    }
}

impl<N: Nodal, E: DirEdge> Debug for HistoryDeque<N, E> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

/// Trait for implementing optional cache usage.
/// 
/// Trait is required for DiGraph, but default implementation does nothing
/// if mut_history() returns None.
pub trait ChangeCache<N: Nodal, E: DirEdge> {

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
