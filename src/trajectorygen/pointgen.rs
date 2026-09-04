use crate::{app::{App, path_gen_info}, trajectorygen::types::*};
use rand::random_range;

///Generate points inside a shape
/// distance based on tool width
pub fn simple(data: &path_gen_info) -> Vec<Point>{

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



    return points;

}

///Randomly generate the points wihin the shape
/// Theoretically could take forever if the points never hit a cell 
pub fn scattershot(data: &path_gen_info) -> Vec<Point>{

    let mut points : Vec<Point> = vec![];

    let pnt_cnt = data.detect_info.get("points_per_shape").unwrap();



    //NOTE: Could be quicker to register every valid point and then just pick from a list 
    //This can be another random

    //For each shape
    for shape in &data.detected_shapes{

        let mut placed = 0.0f64;

        while placed < *pnt_cnt{
            //Place the point in a random spot 
            let genned_point : [usize; 2]= [rand::random_range(shape.min().x()..shape.max().x()), rand::random_range(shape.min().y()..shape.max().y())];


  
            let cell_val = data.difference_map.get_cell_height(genned_point[0], genned_point[1]).unwrap();

            //If invalid cell fire again
            if cell_val == 0.0 || cell_val.is_nan(){
                continue
            }else{
                points.push(Point::create(genned_point[0], genned_point[1]));
                placed += 1.0;
            }

        }
    }


    points

}

///Generate the points using a voronoi diagram approximation to spread them evenly amongst a shape "S"
pub fn voronoi_approx(data : &path_gen_info) -> Vec<Point>{

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


    todo!()

}