/*
Used to generate the heightmaps from given inputs from the tui
*/

use crate::app::{CoreSettings, DeformType};
use rustgeomapping::data_types::heightmap::Heightmap;
use std::collections::HashMap;

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

    let deform_hmap  = match core_settings.deform_type {
        DeformType::LINE => generate_line(deform_hmap, core_settings, deform_settings),
        DeformType::CIRCLE => {todo!()}
        DeformType::RECTANGLE => {todo!()}
        _ => {todo!()}
    };

    return deform_hmap;
}

///Create a line
fn generate_line(
    mut hmap: Heightmap,
    core_settings: &CoreSettings,
    deform_settings: &HashMap<String, f64>,
) -> Heightmap{
    //Precalculate some bits we need
    let len = deform_settings.get("length").unwrap();

    

    let rot_radians = core_settings.deform_rotation.to_radians();
    let sin_rot = rot_radians.sin();
    let cos_rot = rot_radians.cos();

    let hmap_width = hmap.width();
    let hmap_height = hmap.height();


    //Calculate start and end points based on the center, length and rotation
    let start_point = [
        core_settings.deform_center[0] - (cos_rot * len / 2.0),
        core_settings.deform_center[1] - (sin_rot * len / 2.0),
    ];
    let end_point = [
        core_settings.deform_center[0] + (cos_rot * len/2.0),
        core_settings.deform_center[1] + (sin_rot * len/2.0),
    ];

    //Calculate the gradient between the start and end of the line
    let gradient = (end_point[1] - start_point[1]) / (end_point[0] - start_point[0]);
   

    //Create the list of cells
    let mut cells : Vec<(usize, usize, f64)> = vec![];

    //For each mm of thickness - draw another line with the offset
    let mut total_offset = 0.0;
    for i in 0..(core_settings.deform_thickness as usize) {
           
        if i % 2 == 0{
             for j in (start_point[0] as usize .. end_point[0] as usize){
                let y = gradient * j as f64 + start_point[1] as f64;
                cells.push((j, (y + i as f64/2.0)as usize, core_settings.deform_depth));
            }
        }else{
            for j in (start_point[0] as usize .. end_point[0] as usize){
                let y = gradient * j as f64 + start_point[1] as f64;
                cells.push((j as usize, (y - i as f64/2.0)as usize, core_settings.deform_depth));
            }
        }
        
    

    }

    for cell in cells{
        hmap.set_cell_height(cell.1, cell.0, cell.2 as f32);
    }

    return hmap
}
