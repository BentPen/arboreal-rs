pub mod graph_base;
pub mod digraph;

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::{
        digraph::{DiGraph, FileIO}, graph_base::graph_components::* 
    };

    #[derive(PartialEq, Clone, Debug, Default, Serialize, Deserialize)]
    struct NodeExample {
        pub some_id: Id,
        pub other_node_stuff: u8
    }
    impl Nodal for NodeExample {
        fn bare(id: Id) -> Self {
            Self { some_id: id, other_node_stuff: 255 }
        }
        fn node_id(&self) -> Id {
            self.some_id
        }
    }

    #[derive(PartialEq, Clone, Debug, Default, Serialize, Deserialize)]
    struct EdgeExample {
        vertices: (Id, Id),
        other_edge_stuff: Option<String>
    }
    impl DirEdge for EdgeExample {
        fn bare(start: Id, end: Id) -> Self {
            Self { vertices: (start, end), other_edge_stuff: None }
        }
    
        fn terminal_ids(&self) -> (Id, Id) {
            self.vertices
        }
    
        fn change_start(&mut self, new_start: Id) {
            self.vertices.0 = new_start;
        }
    
        fn change_end(&mut self, new_end: Id) {
            self.vertices.1 = new_end;
        }
    }

    #[test]
    fn test_auto_edge_drop() {
        let obj: DiGraph<NodeExample, EdgeExample> = DiGraph::new();
        println!("Default object for example Node/Edge structs:\n{}", obj);
        let file_name = "example.ron";
        let mut example_obj: DiGraph<NodeExample, EdgeExample> = DiGraph::from_terminal_pairs(vec![(1,2), (1,3), (4,3), (3, 5), (5,6), (6,7), (3,7), (8,4), (2,8)]);
        // example_obj.insert_node(NodeExample::bare(7)).unwrap();
        // example_obj.insert_edge_with_nodes(3, 4).unwrap();
        // example_obj.insert_edge_with_nodes(4, 7).unwrap();
        // let start = example_obj.get_node(3).unwrap();
        // assert_eq!(example_obj.get_source(), Ok(start));
        println!("\n{}", example_obj);
        println!("Unreachable from id 1: {:?}", example_obj.nodes_unreachable_from(1));
        println!("Unreachable from id 2: {:?}", example_obj.nodes_unreachable_from(2));
        println!("Unreachable from id 3: {:?}", example_obj.nodes_unreachable_from(3));
        println!("Unreachable from id 42: {:?}\n(Note that the id is not required to be valid)", example_obj.nodes_unreachable_from(42));
        example_obj.save_to_file(file_name).unwrap();
        example_obj.remove_node(3).unwrap();
        // let new_start = example_obj.get_node(4).unwrap();
        // assert_eq!(example_obj.get_source(), Ok(new_start));
        println!("\nAfter dropping id 3:\n{}", example_obj);
        println!("\nUnreachable from id 1: {:?}", example_obj.nodes_unreachable_from(1));
        example_obj.undo().unwrap();
        println!("\nAfter undoing the drop:\n{}", example_obj);
        let new_obj: DiGraph<NodeExample, EdgeExample> = DiGraph::load_from_file(file_name).unwrap();
        println!("\nOriginal object, saved before dropping id 3:\n{}", new_obj);
        std::fs::remove_file(file_name).unwrap_or(());
        // let file_string_string = "example_string_string.ron";
        // let obj_string_string: StateGraph<String, String> = StateGraph::load_or_default(file_string_string);
        // let obj_u8_u16: StateGraph<u8, u16> = StateGraph::load_or_default(file_string_string);
        // println!("Example graph, read as <String, String>:\n{:#?}", obj_string_string);
        // println!("\nSame graph read as <u8, u16> (output below is the default when load_or_default() fails)\n{:#?}", obj_u8_u16);
        // let default_string_string = StateGraph::default();
        // let default_u8_u16 = StateGraph::default();
        // assert_ne!(obj_string_string, default_string_string);
        // assert_eq!(obj_u8_u16, default_u8_u16); // should fail to read Some(String) as Some(u8/u16)
        // let temp_file = "temp_file.ron";
        // let _save_result = obj_string_string.save_to_file(temp_file);
        // let obj2: StateGraph<String, String> = StateGraph::load_or_default(temp_file);
        // assert_eq!(obj_string_string, obj2);
        // std::fs::remove_file(temp_file).unwrap_or(());
    }

    #[test]
    fn test_valid() {
        // let mut obj = StateGraph::default();
        // let node_label = |id: u16| format!("This is node {}", id);
        // let edge_label = |id_start: u16, id_end: u16| format!("Edge from {} to {}", id_start, id_end);
        // let id1 = 7;
        // let id2 = 42;
        // let mut change_vec = vec![];
        // change_vec.push(obj.insert_node(Node::new(id1, Some(node_label(id1)))).unwrap());
        // change_vec.push(obj.insert_node(Node::new(id2, Some(node_label(id2)))).unwrap());
        // change_vec.push(obj.insert_edge(Edge::new(id1, id2, Some(edge_label(id1,id2)))).unwrap());
        // // let connectedness = obj.is_connected();
        // // let validity = obj.is_valid();
        // for change in change_vec.into_iter().enumerate() {
        //     println!("Change {}: {:?}", change.0, change.1);
        // }
        // println!("\n(In,out) degrees by node:");
        // for node in obj.ref_nodes() {
        //     let (in_deg, out_deg) = obj.in_out_degree(node.0).unwrap();
        //     println!("{:>18}:  ({},{})", node.1.clone().unwrap_or(node.0.to_string()), in_deg, out_deg);
        // }
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
    // use crate::{
    //     graph_base::graph_components::*,
    //     digraph::DiGraph, 
    // };

    #[ignore = "run to show usage with adding and removing nodes/edges"]
    #[test]
    fn demo_extend_graph() {
        // let mut obj: StateGraph<u8, u8> = StateGraph::default();
        // { // Initial setup: nodes 0 and 1, with edge
        //     obj.insert_node(Node::new(0, None)).unwrap();
        //     obj.insert_node(Node::new(1, None)).unwrap();
        //     obj.insert_edge(Edge::new(0,1, None)).unwrap();
        //     println!("Initial graph, set as <u8, u8>:\n{:?}", obj);
        // }
        // { // Add node 2, with edge 0->2
        //     obj.insert_node(Node::new(2, None)).unwrap();
        //     obj.insert_edge(Edge::new(0,2,None)).unwrap();
        //     println!("\nGraph with new node + edge:\n{:?}", obj);
        // }
        // { // Add node 3, with edge 2->3
        //     obj.insert_node(Node::new(3, None)).unwrap();
        //     obj.insert_edge(Edge::new(2,3,None)).unwrap();
        //     println!("\nGraph with another new node + edge:\n{:?}", obj);
        // }
        // { // Remove node 0, leaving graph with 2 dangling nodes
        //     obj.remove_node(0).unwrap();
        //     println!("\nGraph with node 0 removed:\n{:?}", obj);
        // }
    }
}
