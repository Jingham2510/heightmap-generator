use ratatui::crossterm::event::{KeyCode, KeyEvent};


use crate::update::update_hmap_gen;

use crate::app::{App, NO_OF_TABS};


pub fn update(app: &mut App, key_event: KeyEvent) {
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
        KeyCode::Enter
            
                if app.tab_no == 0=> {            
                    update_hmap_gen::hmap_gen_parse_input(app, app.curr_input.clone())
                }
        _ => {}
        };
}

