use anyhow::{Error, bail};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use ratatui::{   
    style::Color,
    widgets::{
        Block, BorderType, Borders,
        canvas::{Canvas, Rectangle}
    },
};


use crate::app::{App, DeformType, HeightmapShape};
use rustgeomapping::data_types::heightmap::Heightmap;

use std::env;




pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => app.quit(),
        //Terminal based control
        KeyCode::Char(inp) => {
            app.curr_input.push(inp);
        }
        //Allow the user to delete
        KeyCode::Backspace => {
            app.curr_input.pop();
        }
        //Accept user confirmation
        KeyCode::Enter => parse_user_input(app, app.curr_input.clone()),
        _ => {}
    };
}

///Parse the user command
fn parse_user_input(app: &mut App, user_inp: String) {
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
                            app.deform_type = DeformType::LINE;
                            app.deform_settings = app.deform_type.get_default_settings();
                        }
                        "circle" => {
                            app.deform_type = DeformType::CIRCLE;
                            app.deform_settings = app.deform_type.get_default_settings();
                        }
                        "rectangle" => {
                            app.deform_type = DeformType::RECTANGLE;
                            app.deform_settings = app.deform_type.get_default_settings();
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
                        app.deform_center[0] = new_val.unwrap();
                    } else {
                        return;
                    }
                }
                "posy" => {
                    let new_val = safe_str_to_f64(app, opt_var);
                    if new_val.is_ok() {
                        app.deform_center[1] = new_val.unwrap();
                    } else {
                        return;
                    }
                }

                //Set the shape rotation
                "rot" => {
                    let new_val = safe_str_to_f64(app, opt_var);
                    if new_val.is_ok() {
                        app.deform_rotation = new_val.unwrap();
                    } else {
                        return;
                    }
                }

                //Set the thickness
                "thickness" => {
                    let new_val = safe_str_to_f64(app, opt_var);
                    if new_val.is_ok() {
                        app.deform_thickness = new_val.unwrap();
                    } else {
                        return;
                    }
                }

                //Set the depth
                "depth" => {
                    let new_val = safe_str_to_f64(app, opt_var);
                    if new_val.is_ok() {
                        app.deform_depth = new_val.unwrap();
                    } else {
                        return;
                    }
                }

                //Check if the user is interacting with a shape specific
                _ => {
                    //See if it is a setting in the shapes setting hashmap
                    if app.deform_settings.contains_key(&var) {
                        let new_val = safe_str_to_f64(app, opt_var);
                        if new_val.is_ok() {
                            app.deform_settings.entry(var).insert_entry(new_val.unwrap());
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
        "load" =>{
            //See if a heightmap can be loaded from the filepath            
            let path = format!("{}{}", env::current_dir().unwrap().display(), var);
            let hmap_result = Heightmap::create_from_file(path);
            match hmap_result{
                Ok(hmap) => {
                    app.loaded_hmap = hmap;
                    app.hmap_loaded = true;
                    app.hmap_fp = format!("{}{}", env::current_dir().unwrap().display(), var);

                    get_hmap_info(app);



                },
                Err(e) =>{
                    app.curr_error = String::from("Invalid heightmap filepath");
                    return;
                }
            }
        }

        //Save a heightmap
        "save" =>{
            let path = format!("{}{}", env::current_dir().unwrap().display(), var);
            let result = app.loaded_hmap.save_to_file(&path);
            match result{
                Ok(good) =>{},
                Err(e) => {
                    app.curr_error = String::from("Failed to save heightmap");
                    return;
                }
            }
        }

        _ => {
            app.curr_error = String::from("Unrecognised command!");
            return;
        }
    };

    app.curr_error = String::new();
    app.curr_input = String::from("");
}

///Attempts to parse user input into f64 -> if invalid prints the error to the console
fn safe_str_to_f64(app: &mut App, inp: String) -> Result<f64, anyhow::Error> {
    //Parse the result into an f64 or error
    let inp_f64: Result<f64, _> = inp.parse();

    if inp_f64.is_err() {
        app.curr_error = String::from("Cannot parse variable into a number");
        bail!("Cannot parse into f64!");
    } else {
        return Ok(inp_f64.unwrap());
    }
}



///Calculate the hmap info and canvas
fn get_hmap_info(app: &mut App){
            //Calculate the heightmap stats
            app.hmap_max = app.loaded_hmap.get_max() * 1000.0;
            app.hmap_min = app.loaded_hmap.get_min() * 1000.0;

            app.hmap_cells = vec![];
            let mut row_cnt = 0.0;
            let mut col_cnt = 0.0;                
            //Heightmap drawing function - reverse to draw and match pyplot 
            for row in app.loaded_hmap.cells().into_iter().rev(){
                for cell in row{

                    let cell_colour = calc_cell_colour(app, &cell);

                        app.hmap_cells.push(
                            (col_cnt,
                            row_cnt,
                            cell_colour)
                        );
                    col_cnt += 1.0;
                }   
                col_cnt = 0.0;
                row_cnt += 1.0; 
            }
}

///Calculate what colour the cell should be 
fn calc_cell_colour(app: &mut App, cell_val : &f32) -> Color{

    //Check to see if the cell height is known
    if cell_val.is_nan(){
        return Color::Rgb(255, 255, 255)
    }else{

        let max = app.hmap_max;
        let min = app.hmap_min;
        let range = max - min;
        let median = min + range/2.0;

        //Turn value into mm depth
        let cell_val = cell_val * 1000.0;

        //Calculate the cell value based on distance from the median
        let (r,g,b) = if cell_val <= median{
            (255.0 * (1.0 - ((cell_val - min) / (median - min))), 255.0* (((cell_val - min) / (median - min))), 0.0)
        }else{
            (0.0, 255.0 * (1.0 - ((cell_val - median) / (max - median))), 255.0* (((cell_val - median) / (max - median))))
        };

        

        return Color::Rgb(r as u8, g as u8, b as u8)
    }

}