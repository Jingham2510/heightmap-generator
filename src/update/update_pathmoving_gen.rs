use crate::app::path_gen_info;
use crate::pathgen::DetectionMode;
use crate::pathgen::types::Edge;
///App updating related to path/trajectory generation
use crate::update::update_shared::{calc_cell_colour, safe_str_to_f64};
use crate::pathgen::{edgedetection, types::Direction};



use crate::{
    app::{App}
};
use ratatui::style::Color;
use rustgeomapping::data_types::heightmap::Heightmap;
use rustgeomapping::analysis::analyser::comp_maps;

use std::env;

use anyhow::{Error, bail};


///Parse the user command related to the heightmap generation
pub fn path_gen_parse_input(app: &mut App, cmd_var : Vec<&str>) -> Result<(), anyhow::Error>{

    //Split the command and variables to the important bits
    let (cmd, var) = (
        cmd_var[0].to_ascii_lowercase(),
        cmd_var[1].to_ascii_lowercase(),
    );
    let opt_var = if cmd_var.len() == 3 {
        cmd_var[2].to_ascii_lowercase()
    } else {
        String::from("none")
    };

   
    match cmd.as_str() {

        //Map loading settings
        "load" =>{
             //Set the value of interest
            match var.as_str() {
                "current" => {
                     //See if a heightmap can be loaded from the filepath
                    let path = format!("{}{}", env::current_dir().unwrap().display(), opt_var);
                    let hmap_result = Heightmap::create_from_file(path);
                    match hmap_result {
                        Ok(hmap) => {
                            app.path_gen_info.current_map = hmap;
                            app.path_gen_info.current_map_fp= opt_var;

                        }
                        Err(_e) => {
                        app.curr_error = String::from("Invalid heightmap filepath");
                        bail!("cmd error")
                        }
                    }            
                
                }
                "target" => {
                     //See if a heightmap can be loaded from the filepath
                    let path = format!("{}{}", env::current_dir().unwrap().display(), opt_var);
                    let hmap_result = Heightmap::create_from_file(path);
                    match hmap_result {
                        Ok(hmap) => {
                            app.path_gen_info.target_map = hmap;
                            app.path_gen_info.target_map_fp= opt_var;

                        }
                        Err(_e) => {
                        app.curr_error = String::from("Invalid heightmap filepath");
                        bail!("cmd error")
                    }
                    }
                },
         
            

            _ => {
                app.curr_error = String::from("Invalid load target");
                bail!("cmd error")
            }
            }
        }

        //Seutp the bits and bobs
        "set" =>{
            match var.as_str(){
                //Set the detection mode
                "detmode" =>{
                    match opt_var.as_str() {

                        "testing" => {
                            app.path_gen_info.detect_mode = DetectionMode::TESTING;
                            app.path_gen_info.detect_info = DetectionMode::TESTING.get_default_settings();

                        }

                        _ => {
                            app.curr_error = String::from("invalid detection mode");
                            bail!("Invalid detection mode");
                        }
                    }
                }


                //Check to see if its a mode option
                _ => {
                    //See if it is a setting in the shapes setting hashmap
                    if app.path_gen_info.detect_info.contains_key(&var) {
                        let new_val = safe_str_to_f64(app, opt_var);
                        if new_val.is_ok() {
                            app.path_gen_info.detect_info
                                .entry(var)
                                .insert_entry(new_val.unwrap());
                        } else {
                            bail!("cmd error")
                        }
                    }else if app.path_gen_info.path_info.contains_key(&var) {
                        let new_val = safe_str_to_f64(app, opt_var);
                        if new_val.is_ok() {
                            app.path_gen_info.path_info
                                .entry(var)
                                .insert_entry(new_val.unwrap());
                        } else {
                            bail!("cmd error")
                        }
                    } 
                    else {
                        app.curr_error = String::from("Invalid set option!");
                        bail!("cmd error")
                    }
                }
            }
        }



        //Generate things (difference map, points, path etc)
        "generate" => {

            match var.as_str() {

                //Create the difference map between the provided maps
                "difference" =>{

                    //Check that two maps are loaded
                    if app.path_gen_info.current_map_fp == "" || app.path_gen_info.target_map_fp == ""{
                        app.curr_error = String::from("Load both maps first!");
                        bail!("cmd error")
                    }

                    //Update the heightmap that represents the difference between the target and the current map
                    let result = comp_maps(&app.path_gen_info.current_map, &app.path_gen_info.target_map);
                    
    

                   
                    match result{
                        Ok(map) =>{
                            //Load the map into the path generate structure
                            app.path_gen_info.difference_map = map.clone();

                            app.path_gen_info.difference_width = app.path_gen_info.difference_map.width();
                            app.path_gen_info.difference_height = app.path_gen_info.difference_map.height();



                            //Clear the current cell colours for the canvas
                            app.path_gen_info.diff_map_cells.clear();

                            let max = app.path_gen_info.difference_map.get_max();
                            let min = app.path_gen_info.difference_map.get_min();

                            let mut row_cnt = 0.0;
                            let mut col_cnt = 0.0;
                            //Heightmap drawing function - reverse to draw and match pyplot
                            for row in app.path_gen_info.difference_map.cells().into_iter().rev() {
                                for cell in row {
                                    
                                    let cell_colour = calc_cell_colour(max, min, &cell, 1);

                                    app.path_gen_info.diff_map_cells.push((col_cnt, row_cnt, cell_colour));
                                    col_cnt += 1.0;
                                }
                                col_cnt = 0.0;
                                row_cnt += 1.0;
                            }                            

                            //Flag that a difference map has been generated
                            app.path_gen_info.diff_map_generated = true;
                        }
                        Err(e) =>{
                            app.curr_error = format!("Failed to compare maps - {}", e);
                            bail!(e)
                           }


                    }
                                    

                }

                //Generate the points for the trajectory
                "points" =>{
                    if !app.path_gen_info.diff_map_generated{
                        app.curr_error = String::from("No difference map generated");
                        bail!("no diff map")
                    }

                    match app.path_gen_info.detect_mode{
                        DetectionMode::TESTING =>{
                            //Detect the edges
                            app.path_gen_info.detected_shapes = vec![edgedetection::simple(&app.path_gen_info.difference_map)];

                            //Create the edge shapes
                            app.path_gen_info.edge_cells = create_edge_shapes(app);

                        }
                    }


                }

                _=>{
                    app.curr_error = String::from("Invalid generate option");
                    bail!("cmd error")
                }
            }

        }

        //Save functions (Save difference map/points/trajectory)
        "save" =>{

            match var.as_str() {
                
                //Save the difference map
                "difference" =>{
                    let path = format!("{}{}", env::current_dir().unwrap().display(), opt_var);
                    let result = app.path_gen_info.difference_map.save_to_file(&path);
                    match result {
                        Ok(_good) => {}
                        Err(_e) => {
                            app.curr_error = String::from("Failed to save heightmap");
                            bail!("cmd error")
                        }
                    }

                }

                _ =>{app.curr_error = String::from("Invalid save option");
                    bail!("cmd error")
                    }
            }

        }

        _ =>{
            app.curr_error = String::from("Unrecognised command!");
            bail!("cmd error")
        }
    }

    Ok(())


}



//Go through each edge and determine where to draw the lines
fn create_edge_shapes(app : &App) -> Vec<(f64, f64, Color)>{

    const DRAW_CENTROID : bool = true;

    let mut cells : Vec<(f64, f64, Color)> = vec![];

    //Go through each shape and draw the edge (and the centroid)
    for shape in &app.path_gen_info.detected_shapes{

        //For each edge determine which parts to add to the edge cells
        for edge in shape.edges(){ 
            let (x, mut y) = edge.pos_f64();
            //remembering that its top to bottom for cell rendering
            y = app.path_gen_info.difference_height as f64 - y;

            for dir in edge.dir(){
                match dir{                
                    Direction::NORTH =>{
                        cells.push((x, y - 0.5, Color::Black));
                    }
                    
                    Direction::NORTHEAST =>{
                        cells.push((x + 0.5, y - 0.5, Color::Black));
                    }
                    Direction::EAST =>{
                        cells.push((x + 0.5, y, Color::Black));
                    }
                    Direction::SOUTHEAST =>{
                        cells.push((x + 0.5, y + 0.5, Color::Black));
                    }
                    Direction::SOUTH =>{
                        cells.push((x, y + 0.5, Color::Black));
                    }
                    Direction::SOUTHWEST =>{
                        cells.push((x - 0.5, y + 0.5, Color::Black));
                    }
                    Direction::WEST =>{
                        cells.push((x - 0.5, y, Color::Black));
                    }
                    Direction::NORTHWEST =>{
                        cells.push((x - 0.5, y - 0.5, Color::Black));
                    }                    
                }
            }
        }


        if DRAW_CENTROID{

            //Create the centroid marker
            let (cent_x, mut cent_y) = shape.centre().as_xy_f64();

            //cent_y = app.path_gen_info.difference_height as f64 - cent_y;

            cells.push((cent_x - 0.25, cent_y - 0.25, Color::LightCyan));
            cells.push((cent_x + 0.25, cent_y - 0.25, Color::LightCyan));
            cells.push((cent_x - 0.25, cent_y + 0.25, Color::LightCyan));
            cells.push((cent_x+ 0.25, cent_y + 0.25, Color::LightCyan));
        }


    }

    return cells;

}