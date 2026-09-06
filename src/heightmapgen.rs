/*
Used to generate the heightmaps from given inputs from the tui
*/

use crate::app::{CoreSettings, DeformType};
use rustgeomapping::data_types::heightmap::Heightmap;
use std::collections::HashMap;
use std::f64::consts::PI;

///Generate a heightmap
/// We assume that one pixel is one mm (which it is for the current heightmaps)
pub fn generate_hmap(
    bounds: [[f32; 2]; 2],
    hmap_size: [usize; 2],
    core_settings: &CoreSettings,
    deform_settings: &HashMap<String, f64>,
) -> Heightmap {
    //Create a blank heightmap thats the same size as the loaded one
    //NB: Would be more efficient to calculate the size of the shape first to reduce the number of nans
    let mut deform_hmap = Heightmap::new(hmap_size[0], hmap_size[1]);

    deform_hmap.set_lower_coord_bounds(bounds[0]);
    deform_hmap.set_upper_coord_bounds(bounds[1]);

    if core_settings.deform_thickness == 0.0 {
        return deform_hmap;
    }

    let deform_cells = match core_settings.deform_type {
        DeformType::LINE => generate_line(core_settings, deform_settings),
        DeformType::CIRCLE => {generate_circle(core_settings, deform_settings)}
        DeformType::RECTANGLE => {generate_rectangle(core_settings, deform_settings)}
        _ => {todo!()}
    };

     for cell in deform_cells{
        let _  = deform_hmap.set_cell_height(cell.1, cell.0, cell.2 as f32);
    }

    deform_hmap
}

///Create a line indent (trench)
fn generate_line(
    core_settings: &CoreSettings,
    deform_settings: &HashMap<String, f64>,
) -> Vec<(usize, usize, f64)>{
    //Precalculate some bits we need
    let len = deform_settings.get("length").unwrap();    

    let rot_radians = core_settings.deform_rotation.to_radians();
    let sin_rot = rot_radians.sin();
    let cos_rot = rot_radians.cos();


    //Calculate start and end points  (relative to the center point)
    let start_point = [
        -len / 2.0 ,
         0.0,
    ];
    let end_point = [
        len / 2.0,
        0.0,
    ];

    //Rotate the start and end points
    let start_point = [
        (start_point[0] * cos_rot) - (start_point[1] * sin_rot),
        (start_point[0] * sin_rot) + (start_point[1] * cos_rot),
    ];
    let end_point = [
         (end_point[0] * cos_rot) - (end_point[1] * sin_rot),
        (end_point[0] * sin_rot) + (end_point[1] * cos_rot),
    ];

    //Translate the start and end points
    let start_point = [
        start_point[0] + core_settings.deform_center[0],
        start_point[1] + core_settings.deform_center[1],
    ];
    let end_point = [
        end_point[0] + core_settings.deform_center[0],
        end_point[1] + core_settings.deform_center[1],
    ];


    create_line(start_point, end_point, core_settings.deform_thickness, core_settings.deform_depth)

}

///Create a circular indent
/// NB:Don't need to worry about the rotation for this
fn generate_circle(
    core_settings: &CoreSettings,
    deform_settings: &HashMap<String, f64>,
 ) ->Vec<(usize, usize, f64)>{
    //Get the radius information in mm
    let radius = deform_settings.get("radius").unwrap();


    //Create the empty cells
    let mut cells : Vec<(usize, usize, f64)> = vec![];


    //For the thickness draw each circle a bit further out (starting at radius = radius - thickness/2)
    let mut curr_radius = radius - core_settings.deform_thickness/2.0;

    let center_x = core_settings.deform_center[0];
    let center_y = core_settings.deform_center[1];

    for _i in 0..(core_settings.deform_thickness as usize){

        for j in 0i32..36000{

            //Allow for more resolution in the circle creation
            let j = f64::from(j) * 0.01;

            cells.push((
                (center_x + ((j * (PI / 180.0)).sin() * curr_radius)) as usize,
                (center_y + ((j * (PI / 180.0)).cos() * curr_radius)) as usize,
                core_settings.deform_depth / 1000.0
            ));
        }

        //Increase the radius 
        curr_radius += 1.0;
    }

    cells
}


///Create a rectangle
fn generate_rectangle(
    core_settings: &CoreSettings,
    deform_settings: &HashMap<String, f64>
) -> Vec<(usize, usize, f64)>{

    //Get the width
    let width = deform_settings.get("width").unwrap();

    //Get the length
    let length = deform_settings.get("length").unwrap();

    //Calculate required rotation and trig 
    let rot_radians = core_settings.deform_rotation.to_radians();
    let sin_rot = rot_radians.sin();
    let cos_rot = rot_radians.cos();

    //Calculate each of the rotated corners
    //TL -> TR -> BR -> BL
    let mut corners : [[f64;2];4] = [[-length/2.0, -width/2.0], [length/2.0, -width/2.0],[length/2.0, width/2.0], [-length/2.0, width/2.0]];
 

    for corner in corners.iter_mut(){
        //Rotate the corner point
        corner[0] = (corner[0] * cos_rot) - (corner[1] * sin_rot);
        corner[1] = (corner[0] * sin_rot) + (corner[1] * cos_rot);

        //Offset it by the rectangle center
        corner[0] += core_settings.deform_center[0];



        corner[1] += core_settings.deform_center[1];
    }

    let mut cells : Vec<(usize, usize, f64)> = vec![];
    
    //Create each of the lines
    cells.append(&mut create_line(corners[0], corners[1], core_settings.deform_thickness, core_settings.deform_depth));
    cells.append(&mut create_line(corners[1], corners[2], core_settings.deform_thickness, core_settings.deform_depth));
    cells.append(&mut create_line(corners[3], corners[2], core_settings.deform_thickness, core_settings.deform_depth));
    cells.append(&mut create_line(corners[0], corners[3], core_settings.deform_thickness, core_settings.deform_depth));


    cells
}



///Create a block of cells to represent a line
fn create_line(start_point : [f64; 2], end_point : [f64; 2], thickness : f64, deform_depth : f64) -> Vec<(usize, usize, f64)>{
   
    //Calculate the length
    let length = ((end_point[1] - start_point[1]).powi(2) + (end_point[0] - start_point[0]).powi(2)).sqrt();


   //Calculate the gradient between the start and end of the line
    let gradient = (end_point[1] - start_point[1]) / (end_point[0] - start_point[0]);   
    //Set the deform depth to be in metres
    let deform_depth = deform_depth / 1000.0;

    //Create the list of cells
    let mut cells : Vec<(usize, usize, f64)> = vec![];

    //For each mm of thickness - draw another line with the offset    
    if !gradient.is_infinite(){
    
        for i in 0..(thickness as usize) {

            if i % 2 == 0{
                for j in 0..(length * 10.0) as usize {
                    let j = j as i16 /10;
                    let y = (gradient * j as f64).abs() + start_point[1];
                    cells.push((j as usize + (start_point[0] as usize), (y + i as f64/2.0)as usize, deform_depth));
                }
            }else{
                for j in 0..(length * 10.0) as usize{
                    let j = j as i16 /10;
                    let y = (gradient * j as f64).abs() + start_point[1];
                    cells.push((j as usize + (start_point[0] as usize), (y - i as f64/2.0)as usize, deform_depth));
                }
            }     
        }
    }else{ //Line is pointing straight up

         for i in 0..(thickness as usize) {

            if i % 2 == 0{
                for j in 0..(length * 10.0) as usize{
                    let j = j as i16 /10;
                    let x = start_point[0] + (i as f64/2.0);
                    let y = (j as f64) + start_point[1];
                    cells.push((x as usize, y as usize, deform_depth));
                }
            }else{
                for j in 0..(length * 10.0) as usize{
                    let j = j as i16 /10;
                    let x = start_point[0] - (i as f64/2.0);
                    let y = (j as f64) + start_point[1];
                    cells.push((x as usize, y as usize, deform_depth));
                }
            }     
        }


    }



    cells
}