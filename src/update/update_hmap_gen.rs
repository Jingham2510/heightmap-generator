///Update functions relating to the creation of heightmaps
/// 
/// 
/// 
/// 

use anyhow::bail;

use crate::update::update_shared::{safe_str_to_f64, calc_cell_colour};


use crate::{
    app::{App, DeformType},
    heightmapgen::generate_hmap,
};
use rustgeomapping::data_types::heightmap::Heightmap;

use std::env;


///Parse the user command related to the heightmap generation
pub fn hmap_gen_parse_input(app: &mut App, user_inp: String) {
    //split the user input into command and variable
    let cmd_var: Vec<&str> = user_inp.split(" ").collect();

    if cmd_var.len() != 2 && cmd_var.len() != 3 {
        app.curr_error = String::from(
            "Invalid command! - Command format is [cmd] [variable] [OPTIONAL: variable]",
        );
        return;
    }

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
                            app.gen_info.core_settings.deform_type = DeformType::LINE;
                            app.gen_info.deform_settings =
                                app.gen_info.core_settings.deform_type.get_default_settings();
                        }
                        "circle" => {
                            app.gen_info.core_settings.deform_type = DeformType::CIRCLE;
                            app.gen_info.deform_settings =
                                app.gen_info.core_settings.deform_type.get_default_settings();
                        }
                        "rectangle" => {
                            app.gen_info.core_settings.deform_type = DeformType::RECTANGLE;
                            app.gen_info.deform_settings =
                                app.gen_info.core_settings.deform_type.get_default_settings();
                        }
                        _ => {
                            app.curr_error = format!(
                                "Invalid shape option - Current options are: {:?}",
                                DeformType::VARIANTS
                            );
                            return;
                        }
                    }
                }
                //Set the shape position
                "posx" => {
                    let new_val = safe_str_to_f64(app, opt_var);
                    if new_val.is_ok() {
                        app.gen_info.core_settings.deform_center[0] = new_val.unwrap();
                    } else {
                        return;
                    }
                }
                "posy" => {
                    let new_val = safe_str_to_f64(app, opt_var);
                    if new_val.is_ok() {
                        app.gen_info.core_settings.deform_center[1] = new_val.unwrap();
                    } else {
                        return;
                    }
                }

                //Set the shape rotation
                "rot" => {
                    let new_val = safe_str_to_f64(app, opt_var);
                    if new_val.is_ok() {
                        app.gen_info.core_settings.deform_rotation = new_val.unwrap();
                        
                    } else {
                        return;
                    }
                }

                //Set the thickness
                "thickness" => {
                    let new_val = safe_str_to_f64(app, opt_var);
                    if new_val.is_ok() {
                        app.gen_info.core_settings.deform_thickness = new_val.unwrap();
                    } else {
                        return;
                    }
                }

                //Set the depth
                "depth" => {
                    let new_val = safe_str_to_f64(app, opt_var);
                    if new_val.is_ok() {
                        app.gen_info.core_settings.deform_depth = new_val.unwrap();
                    } else {
                        return;
                    }
                }

                //Set the overlay setting
                "overlay" => {
                    if opt_var == "on" {
                        app.gen_info.overlay_on = true;
                    } else if opt_var == "off" {
                        app.gen_info.overlay_on = false;
                    } else {
                        app.curr_error = String::from("Invalid overlay option!");
                        return;
                    }
                }

                //Set the autogen option
                "autogen" => {                  

                    if opt_var == "on" {
                        app.gen_info.auto_generate = true;
                    } else if opt_var == "off" {
                        app.gen_info.auto_generate = false;
                    } else {
                        app.curr_error = String::from("Invalid autogen option!");
                        return;
                    }
                }

                //Check if the user is interacting with a shape specific
                _ => {
                    //See if it is a setting in the shapes setting hashmap
                    if app.gen_info.deform_settings.contains_key(&var) {
                        let new_val = safe_str_to_f64(app, opt_var);
                        if new_val.is_ok() {
                            app.gen_info.deform_settings
                                .entry(var)
                                .insert_entry(new_val.unwrap());
                        } else {
                            return;
                        }
                    } else {
                        app.curr_error = String::from("Invalid set option!");
                        return;
                    }
                }
            }
        }

        //Load a heightmap
        "load" => {
            //See if a heightmap can be loaded from the filepath
            let path = format!("{}{}", env::current_dir().unwrap().display(), var);
            let hmap_result = Heightmap::create_from_file(path);
            match hmap_result {
                Ok(hmap) => {
                    app.gen_info.loaded_hmap = hmap;
                    app.gen_info.hmap_loaded = true;
                    app.gen_info.hmap_fp = format!("{}{}", env::current_dir().unwrap().display(), var);

                    get_hmap_info(app);
                }
                Err(_e) => {
                    app.curr_error = String::from("Invalid heightmap filepath");
                    return;
                }
            }
        }

        //Save a heightmap
        "save" => match var.as_str() {
            "gen" => {

                let path = format!("{}{}", env::current_dir().unwrap().display(), opt_var);
                let result = app.gen_info.generated_hmap.save_to_file(&path);
                match result {
                    Ok(_good) => {}
                    Err(_e) => {
                        app.curr_error = String::from("Failed to save heightmap");
                        return;
                    }
                }
            }
            "created" => {
                let path = format!("{}{}", env::current_dir().unwrap().display(), opt_var);
                let result = app.gen_info.loaded_hmap.save_to_file(&path);
                match result {
                    Ok(_good) => {}
                    Err(_e) => {
                        app.curr_error = String::from("Failed to save heightmap");
                        return;
                    }
                }
            }

            _ => {
                app.curr_error = String::from("Invalid save option");
                return;
            }
        },

        //Deformation controls (generate, apply)
        "deform" => match var.as_str() {
            //Calculates the deformation shape
            "generate" => {
                app.gen_info.generated_hmap = generate_hmap(
                [app.gen_info.loaded_hmap.lower_coord_bounds(), app.gen_info.loaded_hmap.upper_coord_bounds()],
                [app.gen_info.loaded_hmap.width(), app.gen_info.loaded_hmap.height()],
                &app.gen_info.core_settings,
                &app.gen_info.deform_settings,
                );
                create_generated_cells(app);
            }

            //Apply the generated deformation to the loaded heightmap 
            "apply" => {
                
                //Update the currently loaded heightmap
                app.gen_info.loaded_hmap.update_section(app.gen_info.generated_hmap.clone());

                //Turn the overlay off
                app.gen_info.overlay_on = false;

                //Regenerate the loaded hmap info
                get_hmap_info(app);



            }


            _ => {
                app.curr_error = String::from("Invalid generation command");
                return;
            }
        },

        _ => {
            app.curr_error = String::from("Unrecognised command!");
            return;
        }
    };

    //Automatically update the shape if required
    if cmd == "set" && app.gen_info.overlay_on && app.gen_info.auto_generate {
        app.gen_info.generated_hmap = generate_hmap(
            [
                app.gen_info.loaded_hmap.lower_coord_bounds(),
                app.gen_info.loaded_hmap.upper_coord_bounds(),
            ],
            [app.gen_info.loaded_hmap.width(), app.gen_info.loaded_hmap.height()],
            &app.gen_info.core_settings,
            &app.gen_info.deform_settings,
        );
        create_generated_cells(app);
    }

    app.curr_error = String::new();
    app.curr_input = String::from("");
}



///Calculate the hmap info and canvas
fn get_hmap_info(app: &mut App) {
    app.gen_info.hmap_max = app.gen_info.loaded_hmap.get_max() * 1000.0;
    app.gen_info.hmap_min = app.gen_info.loaded_hmap.get_min() * 1000.0;

    app.gen_info.hmap_cells.clear();
    let mut row_cnt = 0.0;
    let mut col_cnt = 0.0;
    //Heightmap drawing function - reverse to draw and match pyplot
    for row in app.gen_info.loaded_hmap.cells().into_iter().rev() {
        for cell in row {
            let cell_colour = calc_cell_colour(app, &cell);

            app.gen_info.hmap_cells.push((col_cnt, row_cnt, cell_colour));
            col_cnt += 1.0;
        }
        col_cnt = 0.0;
        row_cnt += 1.0;
    }
}


///Create the generated deformation cells
fn create_generated_cells(app: &mut App) {

    app.gen_info.generated_cells.clear();
    let mut row_cnt = 0.0;
    let mut col_cnt = 0.0;
    //Heightmap drawing function - reverse to draw and match pyplot
    for row in app.gen_info.generated_hmap.cells().into_iter().rev() {
        for cell in row {
            //Dont paint nan cells
            if cell.is_nan(){
                col_cnt += 1.0;
                continue;
            }

            let cell_colour = calc_cell_colour(app, &cell);

            app.gen_info.generated_cells.push((col_cnt, row_cnt, cell_colour));
            col_cnt += 1.0;
        }
        col_cnt = 0.0;
        row_cnt += 1.0;
    }
}
