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

    /*
    Approach:
    -Go through cell by cell and find the first top left corner
    -Store the corner as the starting point 
    -From the corner follow the trail of edges (clockwise) until back at the start
    -Then go cell by cell until another top left corner is found (checking it isnt already accounted for)
    -Repeat
    -Filter out every empty shape
        -i.e. remove inner shapes (like the interior of a circle)

     */


    let mut shapes : Vec<DeformShape> = vec![];


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

            //Check if the pixel is a top left corner
            if dirs.contains(&Direction::NORTHWEST){

                //Construct the pixel
                let point = PixelPoint::create(x, y);

                //Check that the pixel doesn't already exist in a detected shapes                
                if shapes.iter().any(|shape |ShapeEdge::in_edge_list(shape.edges(), &point)){
                    continue;
                }               

                //Detect the shape
                shapes.push(detect_shape(hmap, point));
            } 

     
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


///Follow edges clockwise until the starting point is reached to create a shape
fn detect_shape(hmap : &Heightmap, start_point : PixelPoint) -> DeformShape{

    let mut edges : Vec<ShapeEdge> = vec![];

    let mut current_point = start_point;

    loop{

        //Check the surrounding edges
        let dirs = edge_check(hmap, current_point.x() as isize, current_point.y() as isize);

        //Pick the next edge in a clockwise fashion (edge priority)
        let step = clockwise_edge_step(&dirs);

        //Add the current point to the shape
        edges.push(ShapeEdge::create(current_point.x(), current_point.y(), dirs));


        current_point = PixelPoint::create((current_point.x() as isize + step.0) as usize, (current_point.y() as isize + step.1) as usize);

        //If the algo has reached the start point - the shape is complete
        if current_point == start_point{
            break
        }

    }
    

    DeformShape::from(edges)
}

///Tells the shape detector which way to step (clockwise)
fn clockwise_edge_step(dirs : &Vec<Direction>) -> (isize, isize){


    /*Need to think more about this really - how do we traverse an unknown edge?
    Recursive method that checks the surroounding cells to see if they are edges that retracts when it reaches a non edge?
    Although if we can guarantee we are travelling in a clockwise direction correctly we should always be going to an edge spot?
    */


    //The edges indicate which way a shape does not travel (so the check is inverted)
    
    if !dirs.contains(&Direction::NORTH){
        return (0, 1)
    }
    if !dirs.contains(&Direction::NORTHEAST){
        return (1,1)
    }
    if !dirs.contains(&Direction::EAST){
        return (1, 0)
    }
    if !dirs.contains(&Direction::SOUTHEAST){
        return (1,-1)
    }
    if !dirs.contains(&Direction::SOUTH){
        return (0, -1)
    }
    if !dirs.contains(&Direction::SOUTHWEST){
        return (-1, -1)
    }
    if !dirs.contains(&Direction::WEST){
        return (-1, 0)
    }else{ //Doesnt contain an edge in Northwest
        return (-1, -1)
    }

}

