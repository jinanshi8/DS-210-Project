use bproject::data_load;
use bproject::graph_build;
use petgraph::prelude::*;

fn main() {
    
    let mut fb_graph: GraphMap<i64, usize, petgraph::Undirected> = UnGraphMap::new();
    let mut tw_graph: GraphMap<i64, usize, petgraph::Undirected> = UnGraphMap::new();
    let mut ig_graph: GraphMap<i64, usize, petgraph::Undirected> = UnGraphMap::new();
    let fb_df = data_load::load_data("data/facebook.csv");
    let tw_df = data_load::load_data("data/twitter.csv");
    let ig_df = data_load::load_data("data/instagram.csv");
    graph_build::build_the_graph(&fb_df, &mut fb_graph);
    graph_build::build_the_graph(&tw_df, &mut tw_graph);
    graph_build::build_the_graph(&ig_df, &mut ig_graph);

    graph_build::collect_stats(&fb_graph);
    graph_build::collect_stats(&tw_graph);
    graph_build::collect_stats(&ig_graph);
 


}
