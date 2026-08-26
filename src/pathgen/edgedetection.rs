///Methods to detect the edges of earthshapes
use crate::pathgen::types::{Edge, Direction};
use rustgeomapping::data_types::heightmap::Heightmap;

//Assumes that there is only one shape
//Goes through each cell and checks to see if there is a NAN next to it
pub fn simple(hmap : &Heightmap) -> Vec<Edge>{
    
    let mut edges :Vec<Edge> = vec![];
    


    //Go through every column (ignoring edges)
    for i in 1..hmap.width() - 1{
        //Go through every row (ignoring edges)
        for j in 1..hmap.height() - 1{

            //Get the cell value
            let cell_val = hmap.get_cell_height(i, j).unwrap();

            //Check if the cell is nan or zero 
            if cell_val.is_nan() || cell_val == 0.0{
                continue;
            }            

            //Check the surrounding cells
            let dirs = check_surrounding(hmap, i as isize, j as isize);
            if dirs.is_empty(){
                continue;
            }else{
                //If edges exist create the edge object
                edges.push(Edge::create(i, j, dirs))
            }
        }

    }


    return edges
}





///Return the direction(s) of edges if there are any for a cell
fn check_surrounding(hmap : &Heightmap, i : isize, j : isize) -> Vec<Direction>{

    let mut dirs : Vec<Direction> = vec![];

    for row in -1..=1{
        for col in -1..=1{

            let val = hmap.get_cell_height((i + row) as usize, (j + col) as usize).unwrap();

            //Get the cell value
            if val.is_nan() || val == 0.0{

                //Add the corresponding direction
                match (row, col) {
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


    return dirs;
}