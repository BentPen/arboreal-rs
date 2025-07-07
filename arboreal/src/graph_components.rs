
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Node<I: Copy, TN: Clone> (pub I, pub Option<TN>);
impl<I: Copy, TN: Clone> Node<I, TN> {
    pub fn new(id: I, data: Option<TN>) -> Self {
        Self(id, data)
    }
    pub fn bare(id: I) -> Self {
        Self(id, None)
    }
    pub fn empty(&mut self) -> () {
        self.1 = None;
    }
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Edge<I: Copy, TE: Clone> (pub I, pub I, pub Option<TE>);
impl<I: Copy, TE: Clone> Edge<I, TE> {
    pub fn new(id_in: I, id_out: I, data: Option<TE>) -> Self {
        Self(id_in, id_out, data)
    }
    pub fn bare(id_in: I, id_out: I) -> Self {
        Self(id_in, id_out, None)
    }
    pub fn empty(&mut self) -> () {
        self.2 = None;
    }
    pub fn reverse(&mut self) -> () {
        let prev_out = self.1;
        self.1 = self.0;
        self.0 = prev_out;
    }
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum GraphChange<I: Copy, TN: Clone, TE: Clone> {
    AddNode(Node<I, TN>),
    RemoveNode(Node<I, TN>, Vec<Edge<I, TE>>),
    ChangeNodeData(Node<I, TN>),
    AddEdge(Edge<I, TE>),
    AddEdgeWith(Edge<I, TE>, Option<I>, Option<I>),
    RemoveEdge(Edge<I, TE>),
    ChangeEdgeData(Edge<I, TE>),
    InsertNodeAlongEdge(I, Edge<I, TE>),
    Failure(&'static str)
}
impl<I: Copy, TN: Clone, TE: Clone> GraphChange<I, TN, TE> {
    pub fn try_get_edge(&self) -> Result<Edge<I, TE>, &'static str> {
        match self {
            Self::AddEdge(e) => Ok(e.clone()),
            Self::AddEdgeWith(e, _, _) => Ok(e.clone()),
            Self::RemoveEdge(e) => Ok(e.clone()),
            Self::ChangeEdgeData(e) => Ok(e.clone()),
            Self::InsertNodeAlongEdge(_, e) => Ok(e.clone()),
            Self::RemoveNode(_, _) => Err("To get Vec of Edge, call try_get_edge_vec()."),
            Self::Failure(reason) => Err(reason),
            _ => Err("Not an edge variant.")
        }
    }
    pub fn try_get_node(&self) -> Result<Node<I, TN>, &'static str> {
        match self {
            Self::AddNode(n) => Ok(n.clone()),
            Self::RemoveNode(n, _) => Ok(n.clone()),
            Self::ChangeNodeData(n) => Ok(n.clone()),
            Self::Failure(reason) => Err(reason),
            _ => Err("Not a node variant.")
        }
    }
    pub fn try_get_edge_vec(&self) -> Result<Vec<Edge<I, TE>>, &'static str> {
        match self {
            Self::RemoveNode(_, ev) => Ok(ev.clone()),
            Self::Failure(reason) => Err(reason),
            _ => Err("This method only works with the RemoveNode variant.")
        }
    }
    pub fn try_get_edge_with_nodes(&self) -> Result<(Edge<I, TE>, Option<I>, Option<I>), &'static str> {
        match self {
            Self::AddEdgeWith(e, n_in, n_out) => Ok((e.clone(), n_in.clone(), n_out.clone())),
            Self::Failure(reason) => Err(reason),
            _ => Err("This method only works with the AddEdgeWith variant.")
        }
    }
}

