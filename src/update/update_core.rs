use ratatui::crossterm::event::{KeyCode, KeyEvent};


use crate::update::{update_hmap_gen, update_pathmoving_gen};

use crate::app::{App, NO_OF_TABS};

use anyhow::Error;

pub fn update(app: &mut App, key_event: KeyEvent) {

    let mut cmd_ok : Result<(), Error> = Ok(());
    let mut cmd_entered : bool = false;

    match key_event.code {
        //Close the app
        KeyCode::Esc => app.quit(),
        
        //Change the tab number
        KeyCode::Tab=>{
            //Cycle up to the number of tabs then back to 0
            app.tab_no = if app.tab_no == NO_OF_TABS{
                0
            }else{
                app.tab_no + 1
            };

        }

        //Terminal based control
        KeyCode::Char(inp) => {
            app.curr_input.push(inp);
        }
        //Allow the user to delete
        KeyCode::Backspace => {
            app.curr_input.pop();
        }
        //Accept user confirmation
        KeyCode::Enter =>{

                //split the user input into command and variable
                let user_inp = app.curr_input.clone();
                let cmd_var: Vec<&str> = user_inp.split(" ").collect();

                if cmd_var.len() != 2 && cmd_var.len() != 3 {
                    app.curr_error = String::from(
                        "Invalid command! - Command format is [cmd] [variable] [OPTIONAL: variable]",
                    );
                    return;
                }      
            
                //Decide where to pipe the command
                cmd_ok = if app.tab_no == 0{            
                    update_hmap_gen::hmap_gen_parse_input(app, cmd_var)
                }else{
                    update_pathmoving_gen::path_gen_parse_input(app, cmd_var)
                };

                cmd_entered = true;
            }

        _ => {}
        };


    //If the command is okay (no error is thrown) - delete the user input
    if cmd_entered && cmd_ok.is_ok(){
        app.curr_error = String::new();
        app.curr_input = String::from("");
    }

}

