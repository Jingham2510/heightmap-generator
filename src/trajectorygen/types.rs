use std::fs::OpenOptions;
use std::io::Write;
use std::ops::{Add, Div, Range};
use std::collections::HashMap;
use std::f64;

use nalgebra::{matrix, Matrix3, Vector3};
use anyhow::{Error, bail};

///A point which describes a cell location
#[derive(Debug, Default, Clone, Copy)]
pub struct PixelPoint{
    x : usize,
    y : usize,
}

impl PixelPoint{
    ///Create a point
    pub fn create(x : usize, y :usize) -> Self{
        PixelPoint{
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

    pub fn x_f32(&self) ->f32{
        self.x as f32
    }
    pub fn y_f32(&self) ->f32{
        self.y as f32
    }

    //Returns the euclidian distance (l2) between two points
    pub fn eucl_distance(p1 : &PixelPoint, p2 : &PixelPoint) -> f32{
        ((p1.x_f32() - p2.x_f32()).powf(2.0) + (p1.y_f32() - p2.y_f32()).powf(2.0)).sqrt()
    }

    //Take a set of points and turn then into a tuple vector
    //For compatibility with heightmaps
    pub fn destruct_vec_copy(pnts : &Vec<Vec<PixelPoint>>) -> Vec<(usize, usize)>{
        let mut destruct_vec : Vec<(usize, usize)> = vec![];

        for pnt in pnts.iter().flatten(){
            destruct_vec.push((pnt.x(), pnt.y()));
        }

        destruct_vec
    }

}


impl Add for PixelPoint{
    type Output = Self;

    fn add(self, other : Self) -> Self{
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

}

impl Div<usize> for PixelPoint{
    type Output = Self;

    ///Divide the points by a scalar factor 
    fn div(self, rhs: usize) -> Self::Output {
        Self{
            x : self.x / rhs,
            y : self.y / rhs
        }
    }
}

impl PartialEq for PixelPoint{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
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
pub struct ShapeEdge{
    point : PixelPoint,
    dir : Vec<Direction>
}

impl ShapeEdge{
    ///Create an edge object
    pub fn create(x : usize, y: usize, dir : Vec<Direction>) -> Self{
        ShapeEdge{
            point : PixelPoint::create(x, y),
            dir
        }
    }

    pub fn new(x: &usize, y: &usize, dir : Vec<Direction>) -> Self{
        ShapeEdge{
            point : PixelPoint::create(*x, *y),
            dir
        }
    }

    ///Get the position of the edge
    pub fn point(&self) -> PixelPoint{
        self.point
    }


    ///Return the edge directions
    pub fn dir(&self) -> Vec<Direction>{
        self.dir.clone()
    }

    ///Get the position casted to f64
    pub fn pos_f64(&self) -> (f64, f64){
        self.point.as_xy_f64()
    }

    pub fn pos(&self) -> (usize, usize){
        self.point.as_xy()
    }   

    pub fn x(&self) -> usize{
        self.point.x()
    }
    pub fn y(&self) -> usize{
        self.point.y()
    }   

    ///Check to see if a pixel point exists as an edge in a shape
    pub fn in_edge_list(edges : &Vec<ShapeEdge>, point : &PixelPoint) -> bool{
        
        edges.iter().any(|edge| edge.point() == *point)
    }
}



///Turn a list of edges into a searchable hashmap
pub fn edge_vector_to_hash(edges : &Vec<ShapeEdge>) ->HashMap<(usize,usize), Vec<Direction>>{
    let mut edge_hashmap : HashMap<(usize,usize), Vec<Direction>> = HashMap::new();


    for edge in edges{    
        edge_hashmap.insert(
            (edge.point.x, edge.point.y),
            edge.dir.clone()
        );    
    }
    edge_hashmap
}




///A shape that consists of edges and a centre point
#[derive(Default)]
pub struct DeformShape{
    points : Vec<PixelPoint>,
    edges : Vec<ShapeEdge>,
    centre : PixelPoint,
    max : PixelPoint,
    min : PixelPoint
}


impl DeformShape{

    ///Create a shape from a set of edges
    ///Calculates the center as the geometric center (i.e. halfway inbetween the max/min)
    pub fn create(edges : Vec<ShapeEdge>, points : Vec<PixelPoint>) -> Self{


        let mut max_x = 0usize;
        let mut min_x = 1001usize;
        let mut max_y = 0usize;
        let mut min_y = 1001usize;


        for edge in &edges{

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


        let centre_pnt = PixelPoint::create(
            (min_x + max_x)/2 ,
            (min_y + max_y)/2
        );        

        let max_pnt = PixelPoint::create(
            max_x, max_y
        );

        let min_pnt = PixelPoint::create(
            min_x, min_y
        );

        Self {  
                points,
                edges, 
                centre: centre_pnt,
                max : max_pnt,
                min : min_pnt
            }
    }

    ///Return a borrowed set of edges
    pub fn edges(&self) -> &Vec<ShapeEdge>{
        &self.edges
    }

    pub fn all_points(&self) -> Vec<PixelPoint>{
        let mut points = self.points.clone();

        for edge in self.edges.iter(){
            points.push(edge.point())
        }
        points
    }

    pub fn centre(&self) -> &PixelPoint{
        &self.centre
    }

    pub fn max(&self) -> PixelPoint{
        self.max
    }

    pub fn min(&self) -> PixelPoint{
        self.min
    }

    pub fn x_range(&self) -> Range<usize>{
        self.min.x()..self.max.x()
    }

    pub fn y_range(&self) -> Range<usize>{
        self.min.y()..self.max.y()
    }

    pub fn x_range_no_edge(&self) -> Range<usize>{
        (self.min.x() + 1)..(self.max.x() - 1)
    }
    pub fn y_range_no_edge(&self) -> Range<usize>{
        (self.min.y() + 1)..(self.max.y() - 1)
    }

}


///Waypoint representing a real point in the robot world space
#[derive(Debug, Clone, Copy)]
pub struct WayPoint{
    x : f32,
    y : f32,
    z : f32
}

impl WayPoint{

    ///Create a new waypoint
    pub fn new(x: f32, y: f32, z : f32) -> Self{
        WayPoint { x, y, z }
    }

    ///Transform the pixel using known calibration values into a worldspace waypoint
    pub fn from_pixel(point : PixelPoint, depth_m : f32) -> Self{

        const X_ZERO : f32 = -84.96;
        const Y_ZERO : f32 = 2616.13;

        //known precalculated transform points
        const TRANSFORM : Matrix3<f32> = matrix![1.0, 0.0, X_ZERO;
                                               0.0, 1.0, Y_ZERO;
                                               0.0, 0.0, 1.0];

        //Expand the point so it can be multipled by the homogenous transform                                            
        let temp_pnt : Vector3<f32> = Vector3::new(point.x_f32(), point.y_f32(), 1.0);

        //Transform the XY pixels into the world space
        let pnt = TRANSFORM * temp_pnt;

        //The depth is already in the world space (as measured by calibrated heightmaps)
        Self{
            x : pnt[0],
            y : pnt[1],
            //Put the depth into mm
            z : depth_m * 1000.0 
        }      
    }

    ///Create a list of waypoints (organised by shape)
    pub fn from_pixels(points : Vec<Vec<PixelPoint>>, depth : Vec<f32>) -> Result<Vec<Vec<WayPoint>>, anyhow::Error>{

        let mut waypoints : Vec<Vec<WayPoint>> = vec![];
        let mut i = 0;

        for shape in points{

            let mut shape_wp : Vec<WayPoint> = vec![];

            for point in shape{
                shape_wp.push(Self::from_pixel(point, depth[i]));
                i += 1;
            }  

            waypoints.push(shape_wp);          
        }



        Ok(waypoints)
    }

    pub fn x(&self) -> f32{
        self.x
    }

     pub fn y(&self) -> f32{
        self.y
    }

     pub fn z(&self) -> f32{
        self.z
    }



    ///return the euclidian distance between two waypoints    
    pub fn eucl_distance(w1 : &WayPoint, w2 : &WayPoint) -> f32{
        ((w1.x() - w2.x()).powf(2.0) + (w1.y() - w2.y()).powf(2.0) + (w1.z() - w2.z()).powf(2.0)).sqrt()
    }

    ///Export a set of waypoints to a csv
    pub fn export(waypoints : Vec<WayPoint>, filepath : String) -> Result<(), anyhow::Error>{

        //Create or overwrite a file
        let mut file = OpenOptions::new()    
                        .write(true)                   
                        .truncate(true)
                        .create(true)                        
                         .open(filepath)?;


        //Create the waypoint string to be added to the file
        let mut wpnt_string = String::new();

        for wpnt in waypoints{
            wpnt_string.push_str(&format!("({},{},{})\n", wpnt.x, wpnt.y, wpnt.z));
        }
        

        //Write the string into the file
        file.write_all(&wpnt_string.into_bytes())?;


        Ok(())
    }



}