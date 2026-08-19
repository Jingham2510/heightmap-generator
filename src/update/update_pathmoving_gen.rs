///App updating related to path/trajectory generation
use crate::update::update_shared::{self, calc_cell_colour, safe_str_to_f64};


use crate::{
    app::{App}
};
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