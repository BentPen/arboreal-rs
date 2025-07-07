pub mod file;
pub mod graph_components;
pub mod graph_ref;
pub mod cache;
pub mod digraph;
pub mod state_graph;

#[cfg(test)]
mod tests {
    use crate::{
        file::FileIO,
        graph_components::*, 
        digraph::*, 
        state_graph::StateGraph
    };

    #[test]
    fn test_readwrite() {
        let file_string_string = "example_string_string.ron";
        let obj_string_string: StateGraph<String, String> = StateGraph::load_or_default(file_string_string);
        let obj_u8_u16: StateGraph<u8, u16> = StateGraph::load_or_default(file_string_string);
        println!("Example graph, read as <String, String>:\n{:#?}", obj_string_string);
        println!("\nSame graph read as <u8, u16> (output below is the default when load_or_default() fails)\n{:#?}", obj_u8_u16);
        let default_string_string = StateGraph::default();
        let default_u8_u16 = StateGraph::default();
        assert_ne!(obj_string_string, default_string_string);
        assert_eq!(obj_u8_u16, default_u8_u16); // should fail to read Some(String) as Some(u8/u16)
        let temp_file = "temp_file.ron";
        let _save_result = obj_string_string.save_to_file(temp_file);
        let obj2: StateGraph<String, String> = StateGraph::load_or_default(temp_file);
        assert_eq!(obj_string_string, obj2);
        std::fs::remove_file(temp_file).unwrap_or(());
    }

    #[test]
    fn test_valid() {
        let mut obj = StateGraph::default();
        let node_label = |id: u16| format!("This is node {}", id);
        let edge_label = |id_start: u16, id_end: u16| format!("Edge from {} to {}", id_start, id_end);
        let id1 = 7;
        let id2 = 42;
        let mut change_vec = vec![];
        change_vec.push(obj.insert_node(Node::new(id1, Some(node_label(id1)))).unwrap());
        change_vec.push(obj.insert_node(Node::new(id2, Some(node_label(id2)))).unwrap());
        change_vec.push(obj.insert_edge(Edge::new(id1, id2, Some(edge_label(id1,id2)))).unwrap());
        // let connectedness = obj.is_connected();
        // let validity = obj.is_valid();
        for change in change_vec.into_iter().enumerate() {
            println!("Change {}: {:?}", change.0, change.1);
        }
        println!("\n(In,out) degrees by node:");
        for node in obj.ref_nodes() {
            let (in_deg, out_deg) = obj.in_out_degree(node.0).unwrap();
            println!("{:>18}:  ({},{})", node.1.clone().unwrap_or(node.0.to_string()), in_deg, out_deg);
        }
        // println!("Is connected: {}", connectedness);
        // println!("Final validity: {}", validity);

        // let file_path = "test_file.ron";
        // obj.save_to_file(file_path)
        //     .unwrap();
        // let obj2: StateGraph<String, String> = StateGraph::load_from_file(file_path);
        // assert_eq!(obj, obj2);
    }
}

#[cfg(test)]
mod demos {
    use crate::{
        // file::FileIO,
        graph_components::*, 
        digraph::*, 
        state_graph::StateGraph
    };

    #[ignore = "run to show usage with adding and removing nodes/edges"]
    #[test]
    fn demo_extend_graph() {
        let mut obj: StateGraph<u8, u8> = StateGraph::default();
        { // Initial setup: nodes 0 and 1, with edge
            obj.insert_node(Node::new(0, None)).unwrap();
            obj.insert_node(Node::new(1, None)).unwrap();
            obj.insert_edge(Edge::new(0,1, None)).unwrap();
            println!("Initial graph, set as <u8, u8>:\n{:?}", obj);
        }
        { // Add node 2, with edge 0->2
            obj.insert_node(Node::new(2, None)).unwrap();
            obj.insert_edge(Edge::new(0,2,None)).unwrap();
            println!("\nGraph with new node + edge:\n{:?}", obj);
        }
        { // Add node 3, with edge 2->3
            obj.insert_node(Node::new(3, None)).unwrap();
            obj.insert_edge(Edge::new(2,3,None)).unwrap();
            println!("\nGraph with another new node + edge:\n{:?}", obj);
        }
        { // Remove node 0, leaving graph with 2 dangling nodes
            obj.remove_node(0).unwrap();
            println!("\nGraph with node 0 removed:\n{:?}", obj);
        }
    }
}
