/*
Functions that are used to generate the graphs later used in trajectory generation
*/

use crate::trajectorygen::types::WayPoint;

use petgraph::{Graph, graph::NodeIndex, Undirected};


//Create an undirected graph where each waypoint is connected to each waypoint
pub fn create_graph_simple(waypoints : Vec<WayPoint>) -> Graph<WayPoint, f32, Undirected>{

    //Create the undirected graph
    let mut wp_graph  = Graph::<WayPoint, f32, Undirected>::new_undirected();


    //Add the waypoints as nodes
    for waypoint in waypoints{
        wp_graph.add_node(waypoint);
    }

    let mut node_distances : Vec<Vec<f32>> = vec![];

    //Go through each node and calculate the distances distance edge
    for i in 0..wp_graph.node_count(){
        let curr_pnt = &wp_graph[NodeIndex::new(i)];

        let mut curr_distances : Vec<f32> = vec![];

        //Create an edge for every other point
        //The iterator will never need to check indices below itself (as earlier indices will have done it)
        for j in (i+1)..wp_graph.node_count(){
            let target_pnt = &wp_graph[NodeIndex::new(j)];            
            curr_distances.push(WayPoint::eucl_distance(curr_pnt, target_pnt));
        }
        node_distances.push(curr_distances);
    }

    //Go through the distances calculate and add them as edges
    for (i, distances) in node_distances.iter().enumerate(){

        let cnt_start = i + 1;

        for (j, distance) in distances.iter().enumerate(){
            println!("CONNECTING: {i} to {}", j+cnt_start);
            wp_graph.add_edge(NodeIndex::new(i), NodeIndex::new(j + cnt_start), *distance);
        }
    }

    wp_graph
}