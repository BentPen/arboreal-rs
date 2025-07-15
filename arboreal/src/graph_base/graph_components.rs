
use serde::{de::DeserializeOwned, Serialize};

/// Type of identifier for [Nodal] types.
/// 
/// At this point no use case is anticipated needing more than
/// 2^16 = 65536 nodes--but a use case requiring more than
/// 2^8 = 256 is certainly within reason. So this alias is
/// expected to remain for the forseeable future.
/// 
/// In the future this may be redesigned to be a generic.
pub type Id = u16;
pub(crate) const ID_MAX: Id = std::u16::MAX;

/// Trait required for any type used as nodes in a [`DiGraph`](crate::DiGraph) instance.
/// 
/// The user is required to define two methods for this trait:
/// - `Self::bare(id: u16) -> Self` should create a "default" instance of the type, with the provided `id`.
/// - `self.node_id() -> u16` should return a copy of the `id` associated with the instance.
/// 
/// It is incumbent on the library user to ensure the identifier is consistent between these methods.
pub trait Nodal: Clone + PartialEq + Serialize + DeserializeOwned + Default {
    fn bare(id: Id) -> Self;
    fn node_id(&self) -> Id;
}

/// Trait required for any type used as edges in a [`DiGraph`](crate::DiGraph) instance.
/// 
/// The user is required to define four methods for this trait:
/// - `Self::bare(start: u16, end: u16) -> Self` should create a "default" instance of the type, with the provided `start` and `end` terminal identifiers.
/// - `self.terminal_ids() -> (u16, u16)` should return a copy of the `start` and `end` terminal identifiers associated with the instance.
/// - `self.change_start(new_start: u16)` should replace the `start` terminal identifier with `new_start`.
/// - `self.change_end(new_end: u16)` should replace the `end` terminal identifier with `new_end`.
/// 
/// It is incumbent on the library user to have the start and end identifiers be consistent across these methods.
/// 
/// ## Relation to [`Nodal`](crate::Nodal)
/// An edge (an instance of a type implementing `DirEdge`) can exist without its terminal identifiers pointing to `id` values of two nodes (instances of a type
/// implementing [`Nodal`](crate::Nodal)).\
/// The requirement that those identifiers be shared with active nodes is only enforced when the program attempts to add the `DirEdge`
/// type to a [`DiGraph`](crate::DiGraph) instance. If an edge is present in a `DiGraph`, then two nodes (one for each terminal identifier of the `DirEdge`) must also be present.
/// The contrapositive is likewise enforced: whenever a node is removed from a `DiGraph`, then any edges incident on that node are then removed as well.
pub trait DirEdge: Clone + PartialEq + Serialize + DeserializeOwned + Default {

    /// Creates a new instance of the type, without any data besides
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
pub(crate) enum GraphChange<N, E> {
    AddNode(N),
    RemoveNode(N, Vec<E>),
    AddEdge(E),
    AddEdgeWith(E, Option<Id>, Option<Id>),
    RemoveEdge(E),
    InsertNodeAlongEdge(N, E),
    Failure(&'static str)
}
impl<N: Nodal, E: DirEdge> GraphChange<N, E> {
    pub(crate) fn try_get_edge(&self) -> Result<E, &'static str> {
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
    pub(crate) fn try_get_node(&self) -> Result<N, &'static str> {
        match self {
            Self::AddNode(n) => Ok(n.clone()),
            Self::RemoveNode(n, _) => Ok(n.clone()),
            Self::InsertNodeAlongEdge(n, _, ) => Ok(n.clone()),
            Self::Failure(reason) => Err(reason),
            _ => Err("Not a node variant.")
        }
    }
    pub(crate) fn try_get_edge_vec(&self) -> Result<Vec<E>, &'static str> {
        match self {
            Self::RemoveNode(_, ev) => Ok(ev.clone()),
            Self::Failure(reason) => Err(reason),
            _ => Err("This method only works with the RemoveNode variant.")
        }
    }
    pub(crate) fn try_get_edge_with_nodes(&self) -> Result<(E, Option<Id>, Option<Id>), &'static str> {
        match self {
            Self::AddEdgeWith(e, n_in, n_out) => Ok((e.clone(), n_in.clone(), n_out.clone())),
            Self::Failure(reason) => Err(reason),
            _ => Err("This method only works with the AddEdgeWith variant.")
        }
    }
}
