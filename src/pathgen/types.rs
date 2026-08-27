use std::ops::{Add, Div};
 

///A point which describes a cell location
#[derive(Debug, Default, Clone, Copy)]
pub struct Point{
    x : usize,
    y : usize,
}

impl Point{
    ///Create a point
    fn create(x : usize, y :usize) -> Self{
        Point{
            x,
            y
        }
    }

    pub fn as_xy(&self) -> (usize, usize){
        (self.x, self.y)
    }

    pub fn as_xy_f64(&self) -> (f64, f64){
        (self.x as f64, self.y as f64)
    }
}


impl Add for Point{
    type Output = Self;

    fn add(self, other : Self) -> Self{
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

}

impl Div<usize> for Point{
    type Output = Self;

    ///Divide the points by a scalar factor 
    fn div(self, rhs: usize) -> Self::Output {
        Self{
            x : self.x / rhs,
            y : self.y / rhs
        }
    }
}


//Indicates a direction from a cell (where north is up)
#[derive(Debug, Clone, PartialEq)]
pub enum Direction{
    NORTH,
    EAST,
    SOUTH,
    WEST,
    NORTHWEST,
    NORTHEAST,
    SOUTHEAST,
    SOUTHWEST
}

///An edge which describes the cell and the direction(s) of the edge
#[derive(Debug, Clone)]
pub struct Edge{
    cell : Point,
    dir : Vec<Direction>
}

impl Edge{
    ///Create an edge object
    pub fn create(x : usize, y: usize, dir : Vec<Direction>) -> Self{
        Edge{
            cell : Point::create(x, y),
            dir
        }
    }

    ///Get the position of the edge
    pub fn cell(&self) -> Point{
        self.cell
    }


    ///Return the edge directions
    pub fn dir(&self) -> Vec<Direction>{
        self.dir.clone()
    }

    ///Get the position casted to f64
    pub fn pos_f64(&self) -> (f64, f64){
        self.cell.as_xy_f64()
    }

    pub fn pos(&self) -> (usize, usize){
        self.cell.as_xy()
    }

    

}


///A shape that consists of edges and a centre point
pub struct DeformShape{
    edges : Vec<Edge>,
    centre : Point
}

///Create a shape from a set of edges
///Calculates the center as the geometric center (i.e. halfway inbetween the max/min)
impl From<Vec<Edge>> for DeformShape{
    fn from(set : Vec<Edge>) -> Self{


        //Define the comparators
        let x_comp = |edge : &&Edge| {
            edge.pos().0;
        };
        let y_comp = |edge: &&Edge| {
            edge.pos().1;
        };

        //Iterate through every edge to get the max and min points
        let max_x  = set.iter().max_by_key(&x_comp).unwrap().pos().0;
        let max_y = set.iter().max_by_key(&y_comp).unwrap().pos().1;
        let min_x  = set.iter().min_by_key(&x_comp).unwrap().pos().0;
        let min_y = set.iter().min_by_key(&y_comp).unwrap().pos().1; 


        let centre_pnt = Point::create(
            (min_x + max_x)/2 ,
            (min_y + max_y)/2
        );        


        Self {edges: set, centre: centre_pnt}
    }
}

impl DeformShape{

    ///Return a borrowed set of edges
    pub fn edges(&self) -> &Vec<Edge>{
        &self.edges
    }

    pub fn centre(&self) -> &Point{
        &self.centre
    }

}
