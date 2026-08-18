///App updating related to path/trajectory generation
use crate::update::update_shared::{safe_str_to_f64, calc_cell_colour};


use crate::{
    app::{App}
};
use rustgeomapping::data_types::heightmap::Heightmap;

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

        //Create and display the difference between the two maps
        "" => {}

        _ =>{
            app.curr_error = String::from("Unrecognised command!");
            bail!("cmd error")
        }
    }

    Ok(())


}