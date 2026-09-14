
use std::sync::{ mpsc};
use std::sync::mpsc::{Receiver, Sender};




use crate::{app::{PathGenInfo}, trajectorygen::types::*};

use rustgeomapping::data_types::heightmap::Heightmap;

///Generate points inside a shape
/// distance based on tool width
pub fn simple(data: &PathGenInfo) -> Vec<PixelPoint>{

    let mut points : Vec<PixelPoint> = vec![];


    //Due to the method selected 'toolwidth' is a guaranteed key
    let tool_width = data.detect_info.get("tool_width").unwrap();

    let max_count = data.detect_info.get("spacing").unwrap();

   

    //For each detected shape
    for shape in &data.detected_shapes{
      
        //Go through every point in the shape
        for point in shape.all_points(){

            let x = point.x();
            let y = point.y();
            if x % (tool_width/2.0) as usize == 0{
                if (y - shape.min().y()) as f64 % max_count == 0.0{
                    points.push(PixelPoint::create(x, y));
                }
            }
        }
    }



    points

}

///Spread points within a shape on a heightmap
fn spread_points(shape : &DeformShape, no_of_points : u32, map : &Heightmap) -> Vec<PixelPoint>{


    let mut points : Vec<PixelPoint> = vec![];


    let mut placed = 0;

        while placed < no_of_points{
            //Place the point in a random spot 
            let genned_point : [usize; 2]= [rand::random_range(shape.x_range()), rand::random_range(shape.y_range())];

            //If invalid cell fire again
            if !shape.all_points().contains(&PixelPoint::create(genned_point[0], genned_point[1])){
                continue
            }else{
                points.push(PixelPoint::create(genned_point[0], genned_point[1]));
                placed += 1;
            }
        }

        points

}

///Randomly generate the points wihin the shape
/// Theoretically could take forever if the points never hit a cell 
pub fn scattershot(data: &PathGenInfo) -> Vec<PixelPoint>{

    let mut points : Vec<PixelPoint> = vec![];

    let no_of_points = data.detect_info.get("points_per_shape").unwrap();



    //NOTE: Could be quicker to register every valid point and then just pick from a list 
    //This can be another random

    //For each shape
    for shape in &data.detected_shapes{

        let mut shape_pnts = spread_points(shape, *no_of_points as u32, &data.difference_map);

        points.append(&mut shape_pnts);
    }


    points

}


///Voronoi cell structure
#[derive(Default)]
struct VoronoiCell{
    ///The focus point of the cell
    focus : PixelPoint,
    ///The set of points that exist that are closest to this cell
    closest : Vec<PixelPoint>
}

impl From<&PixelPoint> for VoronoiCell{
    fn from(val: &PixelPoint) -> Self {
        VoronoiCell { focus: *val, closest: vec![] }
    }
}


impl VoronoiCell{
    fn get_closest(self) -> Vec<PixelPoint>{
        self.closest
    }

    fn focus(&self) -> PixelPoint{
        self.focus
    }

    fn add_point(&mut self, pnt : PixelPoint){
        self.closest.push(pnt)
    }
}


///Generate the points using a voronoi diagram approximation to spread them evenly amongst a shape "S"
pub fn voronoi_master(data : &PathGenInfo) -> Vec<Vec<PixelPoint>>{


    //Load the user specified values
    let points_per_shape = data.detect_info.get("points_per_shape").unwrap();
    let iterations = data.detect_info.get("iterations").unwrap();


    

    let mut final_points : Vec<Vec<PixelPoint>> = vec![];

    //Need to wait until all threads have completed point calculation
    let mut thread_count = 0;

    let pnt_pipe : (Sender<Vec<PixelPoint>>, Receiver<Vec<PixelPoint>>) = mpsc::channel();

    for shape in &data.detected_shapes    {

        let tx = pnt_pipe.0.clone();

        //Create the voronoi thread (scoped as the data is read only and non-static)
        std::thread::scope(|s| {
            s.spawn(|| {
            
            let pnts = voronoi_gen(&(*iterations as u32), &(*points_per_shape as u32), &shape, &data.difference_map);

            tx.send(pnts);
            });
        });

        //Increase the thread count
        thread_count += 1;
    }


    //Wait for all of the threads to finish
    while thread_count != 0{

        //Read all of the points (order doesn't necessarily matter)
        let pnts = pnt_pipe.1.recv().unwrap();

        final_points.push(pnts);

        thread_count -= 1;
    }
 
    final_points

}


///For a given shape on a given map, generate an approximate equal spread of points
fn voronoi_gen(iterations : &u32, pnts_per_shape : &u32, shape : &DeformShape, map : &Heightmap) -> Vec<PixelPoint>{    
    
    /*
    Overarching plan (LLoyds algorithm -if computationally slow attempt the fortune algorithm?):
        -First, randomly spread the points throughout the shape
        -Then for the number of iterations (or until each point moves a minimal amount)
            -Generate the voronoi diagram
            -find the centroid of each cell
            -Place the point in the center
            -Start again   
     */
    
    
    //Generate the initial random points within the shape
    let mut points = spread_points(shape, *pnts_per_shape, map);

    
    //For the number of iterations specified
    for _i in 0u32..*iterations{

        //Generate the diagram from the points and get the point collections
        let cells = lazy_voronoi_calc(&points, shape);

        //Find the centroid from each cell (potential to sit outside the shape - edge case?)
        let new_points = calc_centroids(cells);

        //Create new points from each centroid
        points = new_points;        
    }

    points
}


///Brute force voronoi cell identification
fn lazy_voronoi_calc(focii : &Vec<PixelPoint>, shape : &DeformShape) -> Vec<VoronoiCell>{
    
    let mut v_cells : Vec<VoronoiCell> = vec![];

    //Create the cells from the focuses (focii?)
    for focus in focii{
        v_cells.push(focus.into())
    }

    //Go through every single point in the shape
    for point in shape.all_points().iter(){

        let x = point.x();
        let y = point.y();
        
        //Create the current point
        let curr_pnt = PixelPoint::create(x, y);

        //Create the default closest point
        let mut cell_index = 0usize;
        let mut dist = 99999.0;

        //Find out which point it is closest to
        for (i, cell) in v_cells.iter().enumerate(){
            let curr_dist = PixelPoint::eucl_distance(&curr_pnt, &cell.focus);
            if  curr_dist< dist{
                cell_index = i;
                dist = curr_dist;
            }
        }

        //Add the point to the current home cell
        v_cells[cell_index].add_point(curr_pnt);        

    }

    v_cells

}

///Calculate centroids as the mean location of all points
fn calc_centroids(cell_points : Vec<VoronoiCell>) -> Vec<PixelPoint>{

    let mut new_centroids : Vec<PixelPoint> = vec![];

    for cell in cell_points{
        let mut x = 0;
        let mut y = 0;

        let mut no_of_points = 0 ;

        for point in cell.get_closest(){
            x += point.x();
            y += point.y();

            no_of_points += 1;
        }

        new_centroids.push(PixelPoint::create(x / no_of_points , y / no_of_points));

    }


    new_centroids

}