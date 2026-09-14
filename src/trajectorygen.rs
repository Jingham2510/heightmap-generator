/*
The tooling used to generate the points in the difference maps and generate trajectories
*/

use std::fmt;
use std::collections::HashMap;


pub mod types;
pub mod edgedetection;
pub mod pointgen;
pub mod graphgen;
pub mod trajgen;


#[derive(Debug, Default)]
pub enum DetectionMode{
    #[default]
    ///Go through every cell and determine whether it needs a point
    SIMPLE,

    ///Randomly generate the points
    SCATTERSHOT,

    ///Iteratively space the points using a voronoi cell method
    VORONOI

}

impl fmt::Display for DetectionMode{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            Self::SIMPLE =>{
                write!(f, "SIMPLE")
            }
            Self::SCATTERSHOT =>{
                write!(f, "SCATTERSHOT")
            }
            Self::VORONOI =>{
                write!(f, "VORONOI")
            }
        }
    }
}

impl DetectionMode{
    ///Get the default hashmap setup for the detection mode
    pub fn get_default_settings(&self) -> HashMap<String, f64>{
        match self{
            DetectionMode::SIMPLE => {
                HashMap::from([(String::from("tool_width"), 50.0f64),(String::from("spacing"), 10.0f64)])
                        }

            DetectionMode::SCATTERSHOT =>{
                HashMap::from([(String::from("points_per_shape"), 50.0f64)])
            }

            DetectionMode::VORONOI =>{
                HashMap::from([(String::from("points_per_shape"), 50.0f64),(String::from("iterations"), 100.0f64)])
            }

        }
    }
}




#[derive(Debug, Default)]
pub enum PathGenMode{
    #[default]
    GRAPH, //Create a graph then do something with it? (unknown yet)
    RAW, //Just save the trajectory as raw points (in the order they were calculated)
    NEARESTNEIGHBOUR, //Order the points based on a nearest neighbour approach
    SHAPENEIGHBOUR //Nearest neighour organised by shape
}

impl fmt::Display for PathGenMode{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            Self::GRAPH =>{
                write!(f, "GRAPH")
            }
            Self::RAW =>{
                write!(f, "RAW")
            }
            Self::NEARESTNEIGHBOUR=>{
                write!(f, "NEAREST NEIGHBOUR")
            }
            Self::SHAPENEIGHBOUR=>{
                write!(f, "NEAREST NEIGHBOUR (BY SHAPE)")
            }
            
        }
    }
}

impl PathGenMode{
    //Get the default hashmap setup for the pathgen mode
    pub fn get_default_settings(&self) -> HashMap<String, f64>{
        match self{
            PathGenMode::NEARESTNEIGHBOUR=> {
                HashMap::from([(String::from("starting_node"), 0.0f64)])
                }

            Self::SHAPENEIGHBOUR=>{
                HashMap::from([(String::from("starting_node"), 0.0f64), (String::from("clearance_height"), 100.0f64)])
            }
            

            //Other options dont require user input
            _ =>{
                HashMap::default()
            }

        }
    }
}

