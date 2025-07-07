
use fixed_deque::Deque;

use crate::graph_components::GraphChange;

#[derive(PartialEq)]
pub struct HistoryDeque<I: Copy, TN: Clone, TE: Clone> (Deque<GraphChange<I, TN, TE>>);

impl<I: Copy, TN: Clone, TE: Clone> HistoryDeque<I, TN, TE> {
    pub fn new(limit: usize) -> Self {
        Self(Deque::new(limit))
    }
}
impl<I: Copy, TN: Clone, TE: Clone> Default for HistoryDeque<I, TN, TE> {
    fn default() -> Self {
        Self::new(100)
    }
}

/// Trait for implementing optional cache usage.
/// 
/// Trait is required for DiGraph, but default implementation does nothing
/// if mut_history() returns None.
pub trait ChangeCache<I: Copy, TN: Clone, TE: Clone> {

    /// Points to deque of GraphChange variants, for ease of undoing operations
    /// 
    /// To disable use of this deque, have this return None
    fn mut_history(&mut self) -> Option<&mut HistoryDeque<I, TN, TE>>;

    /// Erases all entries but keeps the same max length
    fn clear_history(&mut self) {
        if let Some(hist_deq) = self.mut_history() {
            hist_deq.0.clear();
        }
    }

    /// Adds change variant to HistoryDeque; returns oldest item in HistoryDeque if at capacity
    fn register_change(&mut self, change: GraphChange<I, TN, TE>) -> Option<GraphChange<I, TN, TE>> {
        if let Some(hist_deq) = self.mut_history() {
            let first_in = hist_deq.0.push_back(change);
            return first_in;
        }
        None
    }

    /// Removes and returns most recent change variant, or None if HistoryDeque is empty
    fn pop_change(&mut self) -> Option<GraphChange<I, TN, TE>> {
        if let Some(hist_deq) = self.mut_history() {
            return hist_deq.0.pop_back();
        }
        None
    }

}
