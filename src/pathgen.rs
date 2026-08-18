///The tooling used to generate the points in the difference maps and generate trajectories

use std::fmt;


#[derive(Debug, Default)]
pub enum DetectionMode{
    #[default]
    PLACEHOLDER
}

impl fmt::Display for DetectionMode{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            Self::PLACEHOLDER =>{
                write!(f, "PLACEHOLDER")
            }
        }
    }
}



#[derive(Debug, Default)]
pub enum PathGenMode{
    #[default]
    ASTAR
}

impl fmt::Display for PathGenMode{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            Self::ASTAR =>{
                write!(f, "ASTAR")
            }
        }
    }
}

