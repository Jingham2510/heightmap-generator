use std::{sync::{Arc, mpsc::{self, Receiver, Sender}}, thread};

use crate::{app::{PathGenInfo}, trajectorygen::types::*};
use rustgeomapping::data_types::heightmap::Heightmap;

///Generate points inside a shape
/// distance based on tool width
pub fn simple(data: &PathGenInfo) -> Vec<Point>{

    let mut points : Vec<Point> = vec![];


    //Due to the method selected 'toolwidth' is a guaranteed key
    let tool_width = data.detect_info.get("tool_width").unwrap();

    let max_count = data.detect_info.get("spacing").unwrap();

   

    //For each detected shape
    for shape in &data.detected_shapes{

      
        //Go through every cell that sits inside the shapes rectangle
        for x in (shape.min().x()..shape.max().x()).step_by((tool_width/2.0) as usize){
            for y in shape.min().y()..shape.max().y(){

                //Check if there is a depth disparty in the cell
                //If there is depth disparity we know we are inside a shape
                let cell_val = data.difference_map.get_cell_height(x, y).unwrap();
                if cell_val.is_nan() || cell_val == 0.0{
                    continue;
                }


                //Calculate the marker spacing in terms of the local shape coordinates
                if (y - shape.min().y()) as f64 % max_count == 0.0{
                    points.push(Point::create(x, y));
                }
            }
        }
    }



    points

}

///Spread points within a shape on a heightmap
fn spread_points(shape : &DeformShape, no_of_points : u32, map : &Heightmap) -> Vec<Point>{


    let mut points : Vec<Point> = vec![];


    let mut placed = 0;

        while placed < no_of_points{
            //Place the point in a random spot 
            let genned_point : [usize; 2]= [rand::random_range(shape.min().x()..shape.max().x()), rand::random_range(shape.min().y()..shape.max().y())];


  
            let cell_val = map.get_cell_height(genned_point[0], genned_point[1]).unwrap();

            //If invalid cell fire again
            if cell_val == 0.0 || cell_val.is_nan(){
                continue
            }else{
                points.push(Point::create(genned_point[0], genned_point[1]));
                placed += 1;
            }
        }

        points

}

///Randomly generate the points wihin the shape
/// Theoretically could take forever if the points never hit a cell 
pub fn scattershot(data: &PathGenInfo) -> Vec<Point>{

    let mut points : Vec<Point> = vec![];

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

///Generate the points using a voronoi diagram approximation to spread them evenly amongst a shape "S"
pub fn voronoi_approx(data : &PathGenInfo) -> Vec<Point>{

    /*
    Overarching plan (LLoyds algorithm -if computationally slow attempt the fortune algorithm?):
    -For each shape - spin up a thread
    -In each thread:
        -First, randomly spread the points throughout the shape
        -Then for the number of iterations (or until each point moves a minimal amount)
            -Generate the voronoi diagram
            -find the centroid of each cell
            -Place the point in the center
            -Start again   
     */

    //Load the user specified values
    let points_per_shape = data.detect_info.get("points_per_shape").unwrap();
    let iterations = data.detect_info.get("iterations").unwrap();


    
    //Create an Arc of the heightmap so that it can be shared and readable 
    let arc_map = Arc::new(&data.difference_map);

    let mut final_points : Vec<Point> = vec![];

    let mut thread_count = 0;

    let pnt_pipe : (Sender<Vec<Point>>, Receiver<Vec<Point>>) = mpsc::channel();

   
    //Need to wait until all threads have completed point calculation
    for shape in &data.detected_shapes    {

        let shape_clone = shape.clone();
        let it_clone = iterations.clone() as i32;
        let pnt_cnt_clone = points_per_shape.clone() as i32;
        let map_clone = arc_map.clone();
        let send_clone = pnt_pipe.0.clone();

        //Create the voronoi thread
        let _ = thread::spawn(move || {
            
            let pnts = voronoi_gen(it_clone, pnt_cnt_clone, shape_clone, map_clone);

            send_clone.send(pnts);

        });



        //Increase the thread count
        thread_count += 1;
    }


    //Wait for all of the threads to finish
    while thread_count != 0{

        let mut pnts = pnt_pipe.1.recv().unwrap();

        final_points.append(&mut pnts);

        thread_count -= 1;
    }

    todo!()

}



///For a given shape on a given map, generate an approximate equal spread of points
fn voronoi_gen(iterations : i32, pnts_per_shape : i32, shape : &DeformShape, map : Arc<&Heightmap>) -> Vec<Point>{

    //Generate the initial random points

    
    //For the number of iterations specified


    todo!();
}