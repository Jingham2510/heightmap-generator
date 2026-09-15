/*
A collection of methods to generate trajectories from a set of waypoints
*/


use crate::trajectorygen::types::WayPoint;


///Returns the input waypoints in order based on a nearest neighbour approach
pub fn nearest_neighbour(waypoints : Vec<WayPoint>, starting_index : f64, optimise : bool) -> Vec<WayPoint>{

    if waypoints.len() == 1{
        return waypoints
    }
    let starting_index = if starting_index as usize >= waypoints.len(){
         waypoints.len() - 1
    }else{
        starting_index as usize
    };

    //Initialise all nodes as unvisited
    let mut unvisited = waypoints;

    let mut visited : Vec<WayPoint> = vec![];
    
    //Access the starting node
    let mut curr_node = unvisited[starting_index];

    //Move the first node to visited
    visited.push(unvisited.swap_remove(starting_index));


    //While there are still nodes left to visit
    while !unvisited.is_empty(){

        //Calculate all the distances
        let distances = unvisited.iter().map(|x| WayPoint::eucl_distance(&curr_node, x)).collect::<Vec<f32>>();

        //Get the minimum distance indexpt

        let min_distance_index : usize = distances.iter()
                                                  .enumerate()
                                                  .min_by(|(_, a), (_, b)| a.total_cmp(b))
                                                  .map(|(index, _)| index).unwrap();


        //Move the node to visited            
        visited.push(unvisited.swap_remove(min_distance_index));

        curr_node = *visited.last().unwrap();

    }

    //Apply 2-opt if desired
    let wpnts = if !optimise{
        visited
    }else{
        two_opt(visited)
    };

    wpnts

}


///Performs the 2-opt optimisation on a set of waypoints to reduce cross-over
fn two_opt(mut waypoints : Vec<WayPoint>) -> Vec<WayPoint>{

    let mut improvement_found = true;

    //Keep looping until no more optimums can be found
    while improvement_found{
        improvement_found = false;

        //For every node
        for i in 0..waypoints.len() - 1{
            for j in i+2..waypoints.len() - 1{

                //Calculate the change
                let length_delta = -WayPoint::eucl_distance(&waypoints[i], &waypoints[i+1]) 
                                -WayPoint::eucl_distance(&waypoints[j], &waypoints[j+1])
                                +WayPoint::eucl_distance(&waypoints[i+1], &waypoints[j+1])
                                +WayPoint::eucl_distance(&waypoints[i], &waypoints[j]);
                                
                //If the length is changed, swap the path
                if length_delta < 0.0{
                    waypoints[i+1..=j].reverse();
                    improvement_found = true;
                }
            }
        }
    }

    waypoints

}