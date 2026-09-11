/*
A collection of methods to generate trajectories from 
*/


use crate::trajectorygen::types::WayPoint;


///Returns the input waypoints in order based on a nearest neighbour approach
pub fn nearest_neighbour(waypoints : Vec<WayPoint>, starting_index : f64) -> Vec<WayPoint>{

    if waypoints.len() == 1{
        return waypoints
    }

    //Initialise all nodes as unvisited
    let mut unvisited = waypoints;

    let mut visited : Vec<WayPoint> = vec![];
    
    //Access the starting node
    let mut curr_index = starting_index as usize;
    let mut curr_node = unvisited[curr_index as usize];

    //Move the first node to visited
    visited.push(curr_node);
    unvisited.remove(curr_index);


    //While there are still nodes left to visit
    while !unvisited.is_empty(){

        let mut curr_index = 0;
        let mut shortest_dist = 99999.0;

        //Check the distance to every other node
        for (i, node) in unvisited.iter().enumerate(){

            //Calculate the distance
            let dist = WayPoint::eucl_distance(&curr_node, node);

            if dist < shortest_dist{
                curr_index = i;
                shortest_dist = dist;
            }
        }

        //Move the node
        visited.push(unvisited[curr_index]);
        unvisited.remove(curr_index);        
    }

    visited
}