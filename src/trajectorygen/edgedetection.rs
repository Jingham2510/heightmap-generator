///Methods to detect the edges of earthshapes
use crate::trajectorygen::types::{DeformShape, Edge, Direction};
use rustgeomapping::data_types::heightmap::Heightmap;

//Assumes that there is only one shape
//Goes through each cell and checks to see if there is a NAN next to it
pub fn simple(hmap : &Heightmap) -> DeformShape{
    
    let mut edges :Vec<Edge> = vec![];   


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
            let dirs = check_surrounding(hmap, x as isize, y as isize);
            if dirs.is_empty(){
                continue;
            }else{
                //If edges exist create the edge object 
                edges.push(Edge::create(x, y, dirs))
            }
        }

    }

    //Create the shape and return it
    DeformShape::from(edges)
}





///Return the direction(s) of edges if there are any for a cell
fn check_surrounding(hmap : &Heightmap, x : isize, y : isize) -> Vec<Direction>{


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