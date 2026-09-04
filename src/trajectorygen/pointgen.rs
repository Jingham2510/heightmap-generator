use crate::{app::{App, path_gen_info}, trajectorygen::types::*};

///Generate points inside a shape
/// distance based on tool width
pub fn simple(data: &path_gen_info) -> Vec<Point>{

    let mut points : Vec<Point> = vec![];


    //Due to the method selected 'toolwidth' is a guaranteed key
    let tool_width = data.detect_info.get("toolwidth").unwrap();

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



///Generate the points using a voronoi diagram approximation to spread them evenly amongst a shape "S"
pub fn voronoi_approx(data : &path_gen_info) -> Vec<Point>{



    todo!()

}