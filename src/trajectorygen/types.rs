use std::ops::{Add, Div};
use std::collections::HashMap;
use std::f64;

///A point which describes a cell location
#[derive(Debug, Default, Clone, Copy)]
pub struct Point{
    x : usize,
    y : usize,
}

impl Point{
    ///Create a point
    pub fn create(x : usize, y :usize) -> Self{
        Point{
            x,
            y
        }
    }

    pub fn add_x(&mut self, x : usize){
        self.x += x
    }
    pub fn add_y(&mut self, y : usize){
        self.x += y
    }


    pub fn as_xy(&self) -> (usize, usize){
        (self.x, self.y)
    }

    pub fn as_xy_f64(&self) -> (f64, f64){
        (self.x as f64, self.y as f64)
    }

    pub fn x(&self) -> usize{
        self.x
    }
    pub fn y(&self) -> usize{
        self.y
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

    pub fn x(&self) -> usize{
        self.cell.x()
    }
    pub fn y(&self) -> usize{
        self.cell.y()
    }   



}


///Turn a list of edges into a searchable hashmap
pub fn edge_vector_to_hash(edges : &Vec<Edge>) ->HashMap<(usize,usize), Vec<Direction>>{
    let mut edge_hashmap : HashMap<(usize,usize), Vec<Direction>> = HashMap::new();


    for edge in edges{    
        edge_hashmap.insert(
            (edge.cell.x, edge.cell.y),
            edge.dir.clone()
        );    
    }
    return edge_hashmap;
}




///A shape that consists of edges and a centre point
pub struct DeformShape{
    edges : Vec<Edge>,
    centre : Point,
    max : Point,
    min : Point
}

///Create a shape from a set of edges
///Calculates the center as the geometric center (i.e. halfway inbetween the max/min)
impl From<Vec<Edge>> for DeformShape{
    fn from(set : Vec<Edge>) -> Self{


        let mut max_x = 0usize;
        let mut min_x = 1001usize;
        let mut max_y = 0usize;
        let mut min_y = 1001usize;


        for edge in &set{

            if edge.x() > max_x{
                max_x = edge.x()
            }
            if edge.x() < min_x{
                min_x = edge.x()
            }
            if edge.y() > max_y{
                max_y = edge.y()
            }
            if edge.y() < min_y{
                min_y = edge.y()
            }

        }



        /* BUGGED? - not sure!
        //Define the comparators
        let x_comp = |edge : &&Edge| {
            edge.x();
        };
        let y_comp = |edge: &&Edge| {
            edge.y();
        };

        //Iterate through every edge to get the max and min points
        let max_x  = set.iter().max_by_key(x_comp).unwrap().x();        
        let min_x  = set.iter().min_by_key(x_comp).unwrap().x();

        let max_y = set.iter().max_by_key(y_comp).unwrap().y();
        let min_y = set.iter().min_by_key(y_comp).unwrap().y(); 
        */



        let centre_pnt = Point::create(
            (min_x + max_x)/2 ,
            (min_y + max_y)/2
        );        

        let max_pnt = Point::create(
            max_x, max_y
        );

        let min_pnt = Point::create(
            min_x, min_y
        );

        Self {  edges: set, 
                centre: centre_pnt,
                max : max_pnt,
                min : min_pnt
            }
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

    pub fn max(&self) -> Point{
        self.max
    }

    pub fn min(&self) -> Point{
        self.min
    }



}
