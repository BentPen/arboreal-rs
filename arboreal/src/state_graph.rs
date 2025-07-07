// TODO: macroquad gui

use std::collections::HashMap;
use std::fmt::{self, Debug};

use serde::{Serialize, Deserialize};

use crate::graph_components::*;
use crate::cache::*;
use crate::digraph::*;
use crate::file::FileIO;

// Type must implement all of the following:
// Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord, Debug, and Default
type Id = u16;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct StateGraph<TN: Clone, TE: Clone> {

    // Main features
    pub name: Option<String>,
    nodes: Vec<Node<Id, TN>>,
    edges: Vec<Edge<Id, TE>>,

    #[serde(skip)]
    deg_map: HashMap<Id, (usize, usize)>,
    
    // Cached data
    #[serde(skip)]
    undo_history: HistoryDeque<Id, TN, TE>,
}

impl<TN: Clone, TE: Clone> ChangeCache<Id, TN, TE> for StateGraph<TN, TE> {
    fn mut_history(&mut self) -> Option<&mut HistoryDeque<Id, TN, TE>> {
        Some(&mut self.undo_history)
    }
}

impl<TN: Clone, TE: Clone> DiGraph<Id, TN, TE> for StateGraph<TN, TE>
{
    fn ref_nodes(&self) -> &Vec<Node<Id, TN>> {
        &self.nodes
    }
    fn ref_edges(&self) -> &Vec<Edge<Id, TE>> {
        &self.edges
    }
    fn ref_degree_map(&self) -> &HashMap<Id, (usize, usize)> {
        &self.deg_map
    }

    fn mut_nodes(&mut self) -> &mut Vec<Node<Id, TN>> {
        &mut self.nodes
    }
    fn mut_edges(&mut self) -> &mut Vec<Edge<Id, TE>> {
        &mut self.edges
    }
    fn mut_degree_map(&mut self) -> &mut HashMap<Id, (usize, usize)> {
        &mut self.deg_map
    }
}

// // Will be finishing this implementation later to obtain a sweet spot for newlines/tabs
//
// impl<TN: Debug+Clone, TE: Debug+Clone> Display for StateGraph<TN, TE> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let title = match &self.name {
//             Some(name) => format!("StateGraph ('{}')", name),
//             None => "StateGraph".to_string()
//         };
//         write!(f, "{title}\n\t{:?}\n\t{:?}", self.nodes, self.edges)
//     }
// }

impl<TN: Debug+Clone, TE: Debug+Clone> Debug for StateGraph<TN, TE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StateGraph")
            .field("name", &self.name)
            .field("nodes", &self.nodes)
            .field("edges", &self.edges)
            .finish()
    }
}

impl<TN: Clone, TE: Clone> Default for StateGraph<TN, TE> {
    fn default() -> Self {
        let name = None;
        let nodes = Vec::new();
        let edges = Vec::new();
        let undo_history = HistoryDeque::default();
        let deg_map = HashMap::new();
        Self { name, nodes, edges, undo_history, deg_map }
    }
}

impl<TN, TE> FileIO for StateGraph<TN, TE>
where
    TN: Default + Clone + Serialize + for<'a> Deserialize<'a>,
    TE: Default + Clone + Serialize + for<'a> Deserialize<'a>
{
    // Default implementations for
    //  - load_or_default()
    //  - load_from_file()
    //  - save_to_file()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn valid() {
//         let good_options = vec![(0,1), (1,2), (2,3), (0,4), (1,3), (4,3)];
//         let graph: DirectedALGraph<usize> = GraphBuilder::new().edges(good_options).build();
//         let validity = check_dialogue_graph(&graph);
//         println!("{}", validity);
//         assert_eq!(validity, DialogueValidity::Valid { start_index: 0, exit_index: 3 });
//     }
//     #[test]
//     fn invalid_insufficient() {
//         let bad_options: Vec<(usize, usize)> = vec![];
//         let graph: DirectedALGraph<usize> = GraphBuilder::new().edges(bad_options).build();
//         let validity = check_dialogue_graph(&graph);
//         println!("{}", validity);
//         assert_eq!(validity, DialogueValidity::Insufficient);
//     }
//     #[test]
//     fn invalid_no_entry() {
//         let bad_options = vec![(0,1), (1,2), (2,0), (2,3)];
//         let graph: DirectedALGraph<usize> = GraphBuilder::new().edges(bad_options).build();
//         let validity = check_dialogue_graph(&graph);
//         println!("{}", validity);
//         assert_eq!(validity, DialogueValidity::NoEntry);
//     }
//     #[test]
//     fn invalid_multiple_entries() {
//         let bad_options = vec![(0,1), (2,1), (1,3)];
//         let graph: DirectedALGraph<usize> = GraphBuilder::new().edges(bad_options).build();
//         let validity = check_dialogue_graph(&graph);
//         println!("{}", validity);
//         assert_eq!(validity, DialogueValidity::MultipleEntries(0, 2));
//     }
//     #[test]
//     fn invalid_no_exit() {
//         let bad_options = vec![(0,1), (1,2), (2,0), (3,2)];
//         let graph: DirectedALGraph<usize> = GraphBuilder::new().edges(bad_options).build();
//         let validity = check_dialogue_graph(&graph);
//         println!("{}", validity);
//         assert_eq!(validity, DialogueValidity::NoExit);
//     }
//     #[test]
//     fn invalid_multiple_exits() {
//         let bad_options = vec![(0,1), (1,2), (2,3), (0,4), (1,3)];
//         let graph: DirectedALGraph<usize> = GraphBuilder::new().edges(bad_options).build();
//         let validity = check_dialogue_graph(&graph);
//         println!("{}", validity);
//         assert_eq!(validity, DialogueValidity::MultipleExits(3, 4));
//     }
//     #[test]
//     fn invalid_disconnected() {
//         let bad_options = vec![(0,1), (1,2), (4,5), (3,4)];
//         let graph: DirectedALGraph<usize> = GraphBuilder::new().edges(bad_options).build();
//         let validity = check_dialogue_graph(&graph);
//         println!("{}", validity);
//         assert_eq!(validity, DialogueValidity::Disconnected(0, 3));
//     }
// }
