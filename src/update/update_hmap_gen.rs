
use crate::update::update_shared::{safe_str_to_f64, calc_cell_colour};


use crate::{
    app::{App, DeformType},
    heightmapgen::generate_hmap,
};
use rustgeomapping::data_types::heightmap::Heightmap;

use std::env;
use anyhow::bail;


///Parse the user command related to the heightmap generation
pub fn hmap_gen_parse_input(app: &mut App, cmd_var : Vec<&str>) -> Result<(), anyhow::Error>{

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
        //Set the correct values
        "set" => {
            //Set the value of interest
            match var.as_str() {
                //Set the current shape
                "shape" => {
                    //Set the deform type and also the variables
                    match opt_var.as_str() {
                        "line" => {
                            app.hmap_gen_info.core_settings.deform_type = DeformType::LINE;
                            app.hmap_gen_info.deform_settings =
                                app.hmap_gen_info.core_settings.deform_type.get_default_settings();
                        }
                        "circle" => {
                            app.hmap_gen_info.core_settings.deform_type = DeformType::CIRCLE;
                            app.hmap_gen_info.deform_settings =
                                app.hmap_gen_info.core_settings.deform_type.get_default_settings();
                        }
                        "rectangle" => {
                            app.hmap_gen_info.core_settings.deform_type = DeformType::RECTANGLE;
                            app.hmap_gen_info.deform_settings =
                                app.hmap_gen_info.core_settings.deform_type.get_default_settings();
                        }
                        _ => {
                            app.curr_error = format!(
                                "Invalid shape option - Current options are: {:?}",
                                DeformType::VARIANTS
                            );
                            bail!("cmd error")
                        }
                    }
                }
                //Set the shape position
                "posx" => {
                    let new_val = safe_str_to_f64(app, opt_var);
                    if new_val.is_ok() {
                        app.hmap_gen_info.core_settings.deform_center[0] = new_val.unwrap();
                    } else {
                        bail!("cmd error")
                    }
                }
                "posy" => {
                    let new_val = safe_str_to_f64(app, opt_var);
                    if new_val.is_ok() {
                        app.hmap_gen_info.core_settings.deform_center[1] = new_val.unwrap();
                    } else {
                        bail!("cmd error")
                    }
                }

                //Set the shape rotation
                "rot" => {
                    let new_val = safe_str_to_f64(app, opt_var);
                    if new_val.is_ok() {
                        app.hmap_gen_info.core_settings.deform_rotation = new_val.unwrap();
                        
                    } else {
                        bail!("cmd error")
                    }
                }

                //Set the thickness
                "thickness" => {
                    let new_val = safe_str_to_f64(app, opt_var);
                    if new_val.is_ok() {
                        app.hmap_gen_info.core_settings.deform_thickness = new_val.unwrap();
                    } else {
                        bail!("cmd error")
                    }
                }

                //Set the depth
                "depth" => {
                    let new_val = safe_str_to_f64(app, opt_var);
                    if new_val.is_ok() {
                        app.hmap_gen_info.core_settings.deform_depth = new_val.unwrap();
                    } else {
                        bail!("cmd error")
                    }
                }

                //Set the overlay setting
                "overlay" => {
                    if opt_var == "on" {
                        app.hmap_gen_info.overlay_on = true;
                    } else if opt_var == "off" {
                        app.hmap_gen_info.overlay_on = false;
                    } else {
                        app.curr_error = String::from("Invalid overlay option!");
                        bail!("cmd error")
                    }
                }

                //Set the autogen option
                "autogen" => {                  

                    if opt_var == "on" {
                        app.hmap_gen_info.auto_generate = true;
                    } else if opt_var == "off" {
                        app.hmap_gen_info.auto_generate = false;
                    } else {
                        app.curr_error = String::from("Invalid autogen option!");
                        bail!("cmd error")
                    }
                }

                //Check if the user is interacting with a shape specific
                _ => {
                    //See if it is a setting in the shapes setting hashmap
                    if app.hmap_gen_info.deform_settings.contains_key(&var) {
                        let new_val = safe_str_to_f64(app, opt_var);
                        if new_val.is_ok() {
                            app.hmap_gen_info.deform_settings
                                .entry(var)
                                .insert_entry(new_val.unwrap());
                        } else {
                            bail!("cmd error")
                        }
                    } else {
                        app.curr_error = String::from("Invalid set option!");
                        bail!("cmd error")
                    }
                }
            }
        }

        

        //Load a heightmap
        "load" => {

            //Create a blank heightmap
            if var == "blank" {
                app.hmap_gen_info.loaded_hmap = Heightmap::new(999, 999);
                app.hmap_gen_info.hmap_loaded = true;
                get_hmap_info(app);
                return Ok(());
            }


            //See if a heightmap can be loaded from the filepath
            let path = format!("{}{}", env::current_dir().unwrap().display(), var);
            let hmap_result = Heightmap::create_from_file(path);
            match hmap_result {
                Ok(hmap) => {
                    app.hmap_gen_info.loaded_hmap = hmap;
                    app.hmap_gen_info.hmap_loaded = true;
                    app.hmap_gen_info.hmap_fp = format!("{}{}", env::current_dir().unwrap().display(), var);

                    get_hmap_info(app);
                }
                Err(_e) => {
                    app.curr_error = String::from("Invalid heightmap filepath");
                    bail!("cmd error")
                }
            }
        }

        //Save a heightmap
        "save" => match var.as_str() {
            "gen" => {

                let path = format!("{}{}", env::current_dir().unwrap().display(), opt_var);
                let result = app.hmap_gen_info.generated_hmap.save_to_file(&path);
                match result {
                    Ok(_good) => {}
                    Err(_e) => {
                        app.curr_error = String::from("Failed to save heightmap");
                        bail!("cmd error")
                    }
                }
            }
            "created" => {
                let path = format!("{}{}", env::current_dir().unwrap().display(), opt_var);
                let result = app.hmap_gen_info.loaded_hmap.save_to_file(&path);
                match result {
                    Ok(_good) => {}
                    Err(_e) => {
                        app.curr_error = String::from("Failed to save heightmap");
                        bail!("cmd error")
                    }
                }
            }

            _ => {
                app.curr_error = String::from("Invalid save option");
                bail!("cmd error")
            }
        },

        //Deformation controls (generate, apply)
        "deform" => match var.as_str() {
            //Calculates the deformation shape
            "generate" => {
                app.hmap_gen_info.generated_hmap = generate_hmap(
                [app.hmap_gen_info.loaded_hmap.lower_coord_bounds(), app.hmap_gen_info.loaded_hmap.upper_coord_bounds()],
                [app.hmap_gen_info.loaded_hmap.width(), app.hmap_gen_info.loaded_hmap.height()],
                &app.hmap_gen_info.core_settings,
                &app.hmap_gen_info.deform_settings,
                );
                create_generated_cells(app);
            }

            //Apply the generated deformation to the loaded heightmap 
            "apply" => {
                
                //Update the currently loaded heightmap
                let _ = app.hmap_gen_info.loaded_hmap.update_section(app.hmap_gen_info.generated_hmap.clone());

                //Turn the overlay off
                app.hmap_gen_info.overlay_on = false;

                //Regenerate the loaded hmap info
                get_hmap_info(app);



            }


            _ => {
                app.curr_error = String::from("Invalid generation command");
                bail!("cmd error")
            }
        },

        _ => {
            app.curr_error = String::from("Unrecognised command!");
            bail!("cmd error")
        }
    };

    //Automatically update the shape if required
    if cmd == "set" && app.hmap_gen_info.overlay_on && app.hmap_gen_info.auto_generate {
        app.hmap_gen_info.generated_hmap = generate_hmap(
            [
                app.hmap_gen_info.loaded_hmap.lower_coord_bounds(),
                app.hmap_gen_info.loaded_hmap.upper_coord_bounds(),
            ],
            [app.hmap_gen_info.loaded_hmap.width(), app.hmap_gen_info.loaded_hmap.height()],
            &app.hmap_gen_info.core_settings,
            &app.hmap_gen_info.deform_settings,
        );
        create_generated_cells(app);
    }


    Ok(())
}



///Calculate the hmap info and canvas
fn get_hmap_info(app: &mut App) {
    app.hmap_gen_info.hmap_max = app.hmap_gen_info.loaded_hmap.get_max() * 1000.0;
    app.hmap_gen_info.hmap_min = app.hmap_gen_info.loaded_hmap.get_min() * 1000.0;

    app.hmap_gen_info.hmap_cells.clear();
    let mut row_cnt = 0.0;
    let mut col_cnt = 0.0;
    //Heightmap drawing function - reverse to draw and match pyplot
    for row in app.hmap_gen_info.loaded_hmap.cells().into_iter().rev() {
        for cell in row {
            let cell_colour = calc_cell_colour(app.hmap_gen_info.hmap_max, app.hmap_gen_info.hmap_min,&cell, 0);

            app.hmap_gen_info.hmap_cells.push((1000.0 - row_cnt, 1000.0 - col_cnt, cell_colour));
            col_cnt += 1.0;
        }
        col_cnt = 0.0;
        row_cnt += 1.0;
    }
}


///Create the generated deformation cells
fn create_generated_cells(app: &mut App) {

    app.hmap_gen_info.generated_cells.clear();
    let mut row_cnt = 0.0;
    let mut col_cnt = 0.0;
    //Heightmap drawing function - reverse to draw and match pyplot
    for row in app.hmap_gen_info.generated_hmap.cells().into_iter() {
        for cell in row {
            //Dont paint nan cells
            if cell.is_nan(){
                col_cnt += 1.0;
                continue;
            }

            let cell_colour = calc_cell_colour(app.hmap_gen_info.generated_hmap.max(), app.hmap_gen_info.generated_hmap.min(),&cell, 0);

            app.hmap_gen_info.generated_cells.push((row_cnt, 1000.0 - col_cnt, cell_colour));
            col_cnt += 1.0;
        }
        col_cnt = 0.0;
        row_cnt += 1.0;
    }
}
