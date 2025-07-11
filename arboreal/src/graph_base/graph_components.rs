
use serde::{de::DeserializeOwned, Serialize};

pub type Id = u16;

pub trait Nodal: Clone + PartialEq + Serialize + DeserializeOwned + Default {
    fn bare(id: Id) -> Self;
    fn node_id(&self) -> Id;
}

pub trait DirEdge: Clone + PartialEq + Serialize + DeserializeOwned + Default {

    fn bare(start: Id, end: Id) -> Self;
    fn terminal_ids(&self) -> (Id, Id);

    // These functions should check that the id is present in the collection of nodes
    fn change_start(&mut self, new_start: Id);
    fn change_end(&mut self, new_end: Id);

    fn start_id(&self) -> Id {
        self.terminal_ids().0
    }
    fn end_id(&self) -> Id {
        self.terminal_ids().1
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum GraphChange<N: Nodal, E: DirEdge> {
    AddNode(N),
    RemoveNode(N, Vec<E>),
    AddEdge(E),
    AddEdgeWith(E, Option<Id>, Option<Id>),
    RemoveEdge(E),
    InsertNodeAlongEdge(N, E),
    Failure(&'static str)
}
impl<N: Nodal, E: DirEdge> GraphChange<N, E> {
    pub fn try_get_edge(&self) -> Result<E, &'static str> {
        match self {
            Self::AddEdge(e) => Ok(e.clone()),
            Self::AddEdgeWith(e, _, _) => Ok(e.clone()),
            Self::RemoveEdge(e) => Ok(e.clone()),
            Self::InsertNodeAlongEdge(_, e) => Ok(e.clone()),
            Self::RemoveNode(_, _) => Err("To get Vec of DirEdge, call try_get_edge_vec()."),
            Self::Failure(reason) => Err(reason),
            _ => Err("Not an edge variant.")
        }
    }
    pub fn try_get_node(&self) -> Result<N, &'static str> {
        match self {
            Self::AddNode(n) => Ok(n.clone()),
            Self::RemoveNode(n, _) => Ok(n.clone()),
            Self::InsertNodeAlongEdge(n, _, ) => Ok(n.clone()),
            Self::Failure(reason) => Err(reason),
            _ => Err("Not a node variant.")
        }
    }
    pub fn try_get_edge_vec(&self) -> Result<Vec<E>, &'static str> {
        match self {
            Self::RemoveNode(_, ev) => Ok(ev.clone()),
            Self::Failure(reason) => Err(reason),
            _ => Err("This method only works with the RemoveNode variant.")
        }
    }
    pub fn try_get_edge_with_nodes(&self) -> Result<(E, Option<Id>, Option<Id>), &'static str> {
        match self {
            Self::AddEdgeWith(e, n_in, n_out) => Ok((e.clone(), n_in.clone(), n_out.clone())),
            Self::Failure(reason) => Err(reason),
            _ => Err("This method only works with the AddEdgeWith variant.")
        }
    }
}

