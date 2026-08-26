///The tooling used to generate the points in the difference maps and generate trajectories

use std::fmt;
use std::collections::HashMap;


pub mod types;
pub mod edgedetection;


#[derive(Debug, Default)]
pub enum DetectionMode{
    #[default]
    ///Go through every cell and determine whether it needs a point
    TESTING
}

impl fmt::Display for DetectionMode{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            Self::TESTING =>{
                write!(f, "TESTING")
            }
        }
    }
}

impl DetectionMode{
    ///Get the default hashmap setup for the detection mode
    pub fn get_default_settings(&self) -> HashMap<String, f64>{
        match self{
            DetectionMode::TESTING => {
                HashMap::from([(String::from("toolwidth"), 50.0f64)])
            }

            _ => todo!()
        }
    }
}





#[derive(Debug, Default)]
pub enum PathGenMode{
    #[default]
    PLACEHOLDER
}

impl fmt::Display for PathGenMode{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            Self::PLACEHOLDER =>{
                write!(f, "PLACEHOLDER")
            }
        }
    }
}

