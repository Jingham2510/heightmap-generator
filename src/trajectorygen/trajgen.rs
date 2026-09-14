/*
A collection of methods to generate trajectories from a set of waypoints
*/


use crate::trajectorygen::types::WayPoint;


///Returns the input waypoints in order based on a nearest neighbour approach
pub fn nearest_neighbour(waypoints : Vec<WayPoint>, starting_index : f64) -> Vec<WayPoint>{

    if waypoints.len() == 1{
        return waypoints
    }
    let starting_index = if starting_index as usize > waypoints.len(){
         waypoints.len() - 1
    }else{
        starting_index as usize
    };

    //Initialise all nodes as unvisited
    let mut unvisited = waypoints;

    let mut visited : Vec<WayPoint> = vec![];
    
    //Access the starting node
    let mut curr_index = starting_index;
    let mut curr_node = unvisited[curr_index];

    //Move the first node to visited
    visited.push(unvisited.swap_remove(curr_index));


    //While there are still nodes left to visit
    while !unvisited.is_empty(){

        //Calculate all the distances
        let distances = unvisited.iter().map(|x| WayPoint::eucl_distance(&curr_node, x)).collect::<Vec<f32>>();

        //Get the minimum distance index
        let min_distance_index : usize = distances.iter()
                                                  .enumerate()
                                                  .min_by(|(_, a), (_, b)| a.total_cmp(b))
                                                  .map(|(index, _)| index).unwrap();


        //Move the node to visited            
        visited.push(unvisited.swap_remove(min_distance_index));

        curr_node = *visited.last().unwrap();

    }

    visited
}