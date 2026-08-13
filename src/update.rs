use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use anyhow::Error;

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => app.quit(),
        //Terminal based control
        KeyCode::Char(inp) => {app.curr_input.push(inp);},
        //Allow the user to delete 
        KeyCode::Backspace => {app.curr_input.pop();},
        //Accept user confirmation
        KeyCode::Enter =>{parse_user_input(app, app.curr_input.clone())}
        _ => {}
    };
}

///Parse the user command
fn parse_user_input(app: &mut App, user_inp : String){

    //split the user input into command and variable
    let cmd_var : Vec<&str> = user_inp.split(" ").collect();

    if cmd_var.len() != 2 || cmd_var.len() != 3{
        app.curr_error = String::from("Invalid command! - Command format is [cmd] [variable] [OPTIONAL: variable]");
        return
    }else{
        app.curr_error = String::new();
    }

    //Split the command and variables to the important bits
    let (cmd, var) = (cmd_var[0].to_ascii_lowercase().as_str(), cmd_var[1].to_ascii_lowercase().as_str());
    let opt_var = if cmd_var.len() == 3{
        cmd_var[2].to_ascii_lowercase().as_str()
    }else{
        "none"
    };

    match cmd{
        //Set the correct values
        "set" =>{
            //Set the value of interest
            match var{
                //Set the current shape
                "shape" => {

                    match opt_var{
                        _ => {app.curr_error = String::from("Invalid shape option - Current options are: {}")}
                    }


                },
                //Set the shape position
                "posx" => {},
                "posy" => {},
                //Set the shape rotation
                "rotx" => {},
                "roty" => {}

                //Check if the user is interacting with a shape specific
                _ => {}
            
            }

        }
        _ => {app.curr_error = String::from("Unrecognised command!");}
    };


}