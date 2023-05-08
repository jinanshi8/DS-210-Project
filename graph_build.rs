use petgraph::prelude::*;
use polars::{prelude::*, lazy::dsl::concat_lst};

pub fn build_the_graph<'a>(df: &'a DataFrame, the_graph: &mut GraphMap<i64, usize, Undirected>) {
    
    let connections_df = df
        .clone()
        .lazy()
        .select(&[concat_lst([all().alias("con")]).unwrap()])
        .collect()
        .unwrap();
        
    let connections = connections_df.column("con").unwrap();

    for connection_row in connections.iter() {
        match connection_row {
            AnyValue::List(connection_list) => {
                process_connection_list(&connection_list, the_graph);
            }
            _ => {}
        }
    }

      
    
}

fn process_connection_list(
    connection_list: &Series, the_graph: &mut GraphMap<i64, usize, Undirected>
) {
    let mut current_node: i64 = 0;
    for (i, connection) in connection_list.iter().enumerate()
    {
        match connection {
            AnyValue::Int64(con) => {
                if i == 0 {
                    current_node = con;
                }
                else if con == 1{
                    the_graph.add_edge(current_node, con, 1);
                }
            }
            _ => {}
        }
    }
}


pub fn collect_stats(the_graph: &GraphMap<i64, usize, Undirected>) {

    let edges = the_graph.edge_count();
    let nodes = the_graph.node_count();
    println!("Nodes: {}", nodes);
    println!("Edges: {}", edges);

}
