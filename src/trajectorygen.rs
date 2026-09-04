///The tooling used to generate the points in the difference maps and generate trajectories

use std::fmt;
use std::collections::HashMap;


pub mod types;
pub mod edgedetection;
pub mod pointgen;


#[derive(Debug, Default)]
pub enum DetectionMode{
    #[default]
    ///Go through every cell and determine whether it needs a point
    SIMPLE,

    ///Iteratively space the points using a voronoi cell method
    VORONOI

}

impl fmt::Display for DetectionMode{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            Self::SIMPLE =>{
                write!(f, "SIMPLE")
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

            DetectionMode::VORONOI =>{
                HashMap::from([(String::from("point_count"), 50.0f64),(String::from("iterations"), 100.0f64)])
            }

            _ => todo!()
        }
    }
}




#[derive(Debug, Default)]
pub enum PathGenMode{
    #[default]
    TESTING
}

impl fmt::Display for PathGenMode{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            Self::TESTING =>{
                write!(f, "PLACEHOLDER")
            }
        }
    }
}

