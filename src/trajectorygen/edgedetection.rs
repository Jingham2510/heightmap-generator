use std::sync::Arc;

///Methods to detect the edges of earthshapes
use crate::trajectorygen::types::{DeformShape, Direction, PixelPoint, ShapeEdge};
use rustgeomapping::data_types::heightmap::Heightmap;

//Assumes that there is only one shape
//Goes through each cell and checks to see if there is a NAN next to it
pub fn simple(hmap : &Heightmap) -> DeformShape{
    
    let mut edges :Vec<ShapeEdge> = vec![];   


    //Go through every column (ignoring edges)
    for x in 1..hmap.width() - 1{
        //Go through every row (ignoring edges)
        for y in 1..hmap.height() - 1 {

            //Get the cell value
            let cell_val = hmap.get_cell_height(x, y).unwrap();

            //Ignore if the cell is nan or zero 
            if cell_val.is_nan() || cell_val == 0.0{
                continue;
            }            

            //Check the surrounding cells
            let dirs = edge_check(hmap, x as isize, y as isize);
            if dirs.is_empty(){
                continue;
            }else{
                //If edges exist create the edge object 
                edges.push(ShapeEdge::create(x, y, dirs))
            }
        }

    }

    //Create the shape and return it
    DeformShape::from(edges)
}





///Return the direction(s) of edges if there are any for a cell
fn edge_check(hmap : &Heightmap, x : isize, y : isize) -> Vec<Direction>{


    let mut dirs : Vec<Direction> = vec![];

    //check the surrounding cells
    for width_mod in -1..=1{
        for height_mod in -1..=1{

            let val = hmap.get_cell_height((x + width_mod) as usize, (y + height_mod) as usize).unwrap();

            //Get the cell value
            if val.is_nan() || val == 0.0{

                //Add the corresponding direction
                match (height_mod, width_mod) {
                    (-1, -1) =>{
                        dirs.push(Direction::NORTHWEST)
                    }
                    (-1, 0) =>{
                        dirs.push(Direction::NORTH)
                    }
                    (-1, 1) =>{
                        dirs.push(Direction::NORTHEAST)
                    }
                    (0, -1) =>{
                        dirs.push(Direction::WEST)
                    }
                    (0,0) => {panic!("Self is NAN?")}
                    (0, 1) =>{
                        dirs.push(Direction::EAST)
                    }
                    (1, -1) => {
                        dirs.push(Direction::SOUTHWEST)
                    }
                    (1, 0) =>{
                        dirs.push(Direction::SOUTH)
                    }
                    (1, 1) =>{
                        dirs.push(Direction::SOUTHEAST)
                    }

                    _ => {panic!("Invalid")}
                }


            }




        }
    }


    dirs
}



///Detects and seperates different shapes within a difference map
/// Diagonals are not connected
/// Will eventually be the default edge detection paradigm (why let the user choose a simpler detection method?)
pub fn multiple(hmap :&Heightmap) -> Vec<DeformShape>{

    let mut shapes : Vec<DeformShape> = vec![];

    //Put the heightmap on the heap to save memory
    let heaped_hmap = Box::new(hmap);

    let mut visit_map = Box::new(Heightmap::new(hmap.width(), hmap.height()));

     //Go through every column (ignoring edges)
    for x in 1..hmap.width() - 1{
        //Go through every row (ignoring edges)
        for y in 1..hmap.height() - 1 {          
           
            //Check if the cell has been visited
            if !visit_map.get_cell_height(x, y).unwrap().is_nan(){
                continue;
            }

           //Get the cell value
            let cell_val = hmap.get_cell_height(x, y).unwrap();

            //Ignore if the cell is nan or zero 
            if cell_val.is_nan() || cell_val == 0.0{
                continue;
            }

            let mut edge_set : Vec<ShapeEdge> = vec![];

            flood_fill(&heaped_hmap, &mut visit_map, &mut edge_set, PixelPoint::create(x, y));

            shapes.push(DeformShape::from(edge_set));
            } 

     
        }

    
    
    //Remove empty shape - double check this should ignore edges
    shapes.retain(|shape| {
        let mut ans = true;
        for x in shape.x_range_no_edge(){
            for y in shape.y_range_no_edge(){
                let cell_val = hmap.get_cell_height(x, y).unwrap();

                if !cell_val.is_nan() && cell_val != 0.0{
                    ans = false;
                    break;
                }
            }
            if !ans{
                break
            }
        }
        ans 
    });

    println!("No of shapes: {}", shapes.len());

    shapes
}





/*
Online solution seems to be:
(I would require a copy of the map)
Start from the first cell spotted and flood fill recursively until you have marked all cells as visited 
then continue iterating until a non-visited cell is spotted (This is useful for just counting the number of islands)

I think it could be modified such that everytime you spot a non-visited cell
create a new edge list
pass that shape as a reference
When a cell is marked as visited -> it is also checked if it is an edge, which are then added to the edge list

*/

fn flood_fill(map : &Heightmap, visit_map : &mut Heightmap, current_edges : &mut Vec<ShapeEdge>, curr_point : PixelPoint){

    let x = curr_point.x();
    let y = curr_point.y();

    //Check that the point is valid
    if  x >= visit_map.width()  || y >= visit_map.height(){
        return
    }
    //Check if the points has been visited
    if !visit_map.get_cell_height(x, y).unwrap().is_nan(){
        return
    }
    
    //Get the cell value
    let height = map.get_cell_height(x, y).unwrap();
    
    //Check that there is a cell there
    if height.is_nan() || height == 0.0{
        return
    }

    //Add any edges to the edge list
    current_edges.push(ShapeEdge::new(&x, &y, edge_check(map, x as isize, y as isize)));

    //Mark the spot as visited
    visit_map.set_cell_height(x, y, 1.0);

    //Traverse to the cells around
    flood_fill(map, visit_map, current_edges, PixelPoint::create(x + 1, y));
    flood_fill(map, visit_map, current_edges, PixelPoint::create(x - 1, y));
    flood_fill(map, visit_map, current_edges, PixelPoint::create(x , y + 1));
    flood_fill(map, visit_map, current_edges, PixelPoint::create(x , y - 1));




    

}
