
use std::collections::HashMap;
use std::hash::Hash;

use crate::graph_components::{Node, Edge, GraphChange};
use crate::graph_ref;
use crate::cache::ChangeCache;

// Type aliases for frequently used collections
type NV<I, TN> = Vec<Node<I, TN>>;
type EV<I, TE> = Vec<Edge<I, TE>>;
type DM<I> = HashMap<I, (usize, usize)>;

pub trait DiGraph<I, TN, TE>: ChangeCache<I, TN, TE> + Default
where
    I: Copy + PartialEq + Default + Eq + Hash,
    TN: Clone,
    TE: Clone
{

    fn ref_nodes(&self) -> &NV<I, TN>;
    fn ref_edges(&self) -> &EV<I, TE>;
    fn ref_degree_map(&self) -> &DM<I>;
    fn mut_nodes(&mut self) -> &mut NV<I, TN>;
    fn mut_edges(&mut self) -> &mut EV<I, TE>;
    fn mut_degree_map(&mut self) -> &mut DM<I>;

    /// Inserts `node` into graph, with no edges.
    /// 
    /// If the node's id is already in use, an error is returned.
    fn insert_node(&mut self, node: Node<I, TN>) -> Result<(), &'static str> {
        let new_node = graph_ref::check_add_node::<I, TN, TE>(self.ref_nodes(), node).try_get_node()?;
        let node_id = new_node.0;
        self.register_change(GraphChange::AddNode(new_node.clone()));
        self.mut_nodes().push(new_node);
        self.mut_degree_map().insert(node_id, (0, 0));
        Ok(())
    }

    /// Inserts `edge` into graph.
    /// 
    /// If the edge's terminal nodes are not present in the graph,
    /// or an edge with these same terminals is already present in the graph,
    /// an error is returned.
    fn insert_edge(&mut self, edge: Edge<I, TE>) -> Result<(), &'static str> {
        let new_edge = graph_ref::check_add_edge::<I, TN, TE>
            (self.ref_nodes(), self.ref_edges(), edge)
            .try_get_edge()?;
        let (id_before, id_after) = (new_edge.0, new_edge.1);
        self.register_change(GraphChange::AddEdge(new_edge.clone()));
        self.mut_edges().push(new_edge);
        let deg_map = self.mut_degree_map();
        // Increase start node's out-degree by 1
        deg_map.get_mut(&id_before)
            .unwrap()
            .1 += 1;
        // Increase end node's in-degree by 1
        deg_map.get_mut(&id_after)
            .unwrap()
            .0 += 1;
        Ok(())
    }

    /// Inserts a bare `Node` with id `new_id` along edge from `id_before` to `id_after`,\
    /// deleting the original edge and creating two new edges.
    /// 
    /// That edge's data is moved to the new edge from `id_before` to `new_id`.
    /// 
    /// If the old edge does not exist, or `new_id` is already in use, an error is returned.
    fn insert_node_along(&mut self, new_id: I, id_before: I, id_after: I) -> Result<(), &'static str> {
        let new_node = graph_ref::check_add_node::<I, TN, TE>
            (self.ref_nodes(), Node::bare(new_id))
            .try_get_node()?;
        let old_edge = graph_ref::check_remove_edge::<I, TN, TE>
            (self.ref_edges(), id_before, id_after)
            .try_get_edge()?;
        let edge_index = graph_ref::edge_index::<I, TE>
            (self.ref_edges(), id_before, id_after)
            .unwrap();
        let edge_before = Edge::new(id_before, new_id, old_edge.2.clone());
        let edge_after = Edge::bare(new_id, id_after);
        self.register_change(GraphChange::InsertNodeAlongEdge(new_id, old_edge));
        self.mut_nodes()
            .push(new_node);
        let edges = self.mut_edges();
        edges.remove(edge_index);
        edges.push(edge_before);
        edges.push(edge_after);

        self.mut_degree_map().insert(new_id, (1,1));
        // degrees of id_before and id_after will be unchanged

        Ok(())
    }

    /// Inserts a bare `Edge` with provided terminals, creating bare nodes at those terminals if needed.
    /// 
    /// If an edge with these terminals already exists, an error is returned.
    fn insert_edge_with_nodes(&mut self, id_in: I, id_out: I) -> Result<(), &'static str> {
        let change =
            graph_ref::check_add_edge_with_nodes::<I, TN, TE>(self.ref_nodes(), self.ref_edges(), id_in, id_out);
        let (new_edge, new_in, new_out) = change.try_get_edge_with_nodes()?;
        if let Some(new_id) = new_in {
            self.insert_node(Node::bare(new_id)).unwrap();
        }
        if let Some(new_id) = new_out {
            self.insert_node(Node::bare(new_id)).unwrap();
        }
        self.insert_edge(new_edge).unwrap();
        self.register_change(change);
        Ok(())
    }

    /// Returns Some((in_degree, out_degree)) from internally maintained degree map, or None if node index not found
    fn in_out_degree(&self, id: I) -> Option<(usize, usize)> {
        if let Some(degs) = self.ref_degree_map().get(&id) {
            return Some(*degs);
        }
        None
    }
    /// Convenience method to return only part of `self.in_out_degree(id)`
    fn in_degree(&self, id: I) -> Option<usize> {
        if let Some((deg_in, _deg_out)) = self.in_out_degree(id) {
            return Some(deg_in);
        }
        None
    }
    /// Convenience method to return only part of `self.in_out_degree(id)`
    fn out_degree(&self, id: I) -> Option<usize> {
        if let Some((_deg_in, deg_out)) = self.in_out_degree(id) {
            return Some(deg_out);
        }
        None
    }
    
    /// Returns Id vec of nodes with in_degree 0
    /// 
    /// In the future, this method may be revoked in favor of a combined sink/source\
    /// method that returns (Vec<I>, Vec<I>)
    fn source_node_ids(&self) -> Vec<I> {
        let mut ids = Vec::with_capacity(self.ref_nodes().len());
        for (id, (in_deg, _out_deg)) in self.ref_degree_map().iter() {
            if *in_deg == 0 {
                ids.push(*id);
            }
        }
        ids.shrink_to_fit();
        ids
    }

    fn get_source(&self) -> Result<I, &'static str> {
        let mut sources = self.source_node_ids();
        match sources.len() {
            1 => Ok(sources.pop().unwrap()),
            0 => Err("No sources in graph."),
            _ => Err("Multiple sources in graph.")
        }
    }

    /// Returns Id vec of nodes with out_degree 0
    /// 
    /// In the future, this method may be revoked in favor of a combined sink/source\
    /// method that returns (Vec<I>, Vec<I>)
    fn sink_node_ids(&self) -> Vec<I> {
        let mut ids = Vec::with_capacity(self.ref_nodes().len());
        for (id, (_in_deg, out_deg)) in self.ref_degree_map().iter() {
            if *out_deg == 0 {
                ids.push(*id);
            }
        }
        ids.shrink_to_fit();
        ids
    }

    fn remove_node(&mut self, id: I) -> Result<(), &'static str> {
        let change =
            graph_ref::check_remove_node::<I, TN, TE>(self.ref_nodes(), self.ref_edges(), id);
        let out_node_id = change.try_get_node()?
            .0;
        let out_edges = change.try_get_edge_vec()?;
        let index =
            graph_ref::node_index::<I, TN>(self.ref_nodes(), id)
            .unwrap();
        self.register_change(change);
        for edge in out_edges {
            let _partial_change = self.remove_edge_unregistered(edge.0, edge.1)?;
        }
        self.mut_degree_map()
            .remove(&out_node_id);
        self.mut_nodes()
            .remove(index);
        Ok(())
    }

    fn remove_edge(&mut self, id_in: I, id_out: I) -> Result<(), &'static str> {
        let change = self.remove_edge_unregistered(id_in, id_out)?;
        self.register_change(change);
        Ok(())
    }

    fn remove_edge_unregistered(&mut self, id_in: I, id_out: I) -> Result<GraphChange<I, TN, TE>, &'static str> {
        let change = 
            graph_ref::check_remove_edge::<I, TN, TE>(self.ref_edges(), id_in, id_out);
        let out_edge = change.try_get_edge()?;
        let index =
            graph_ref::edge_index::<I, TE>(self.ref_edges(), id_in, id_out)
            .unwrap();
        self.mut_degree_map()
            .get_mut(&out_edge.0)
            .unwrap()
            .1 -= 1;
        self.mut_degree_map()
            .get_mut(&out_edge.1)
            .unwrap()
            .0 -= 1;
        self.mut_edges()
            .remove(index);
        Ok(change)
    }

    fn path_from_source(&self, id: I) -> Result<Vec<I>, &'static str> {
        let source = self.get_source()?;
        todo!()
    }
    fn path_to_sink(&self, id: I) -> Result<Vec<I>, &'static str> {
        todo!()
    }

    fn is_connected(&self) -> bool {
        todo!()
    }
    fn is_terminable(&self) -> bool {
        todo!()
    }
    fn is_valid(&self) -> bool {
        self.is_connected() && self.is_terminable()
    }

    fn from_edges(edges: Vec<(I, I)>) -> Self {
        let mut instance = Self::default();
        for edge in edges {
            instance.insert_edge_with_nodes(edge.0, edge.1).unwrap();
        }
        instance.clear_history();
        instance
    }

}
