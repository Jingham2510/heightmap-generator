///A collection of widgets used in the 
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::app::App;
use crate::ui::ui_shared;


///Base creation of the pathmoving page
pub fn pathmoving_core(app : &mut App, frame : &mut Frame, widget : Rect){


    //Create the core layout of the pathmoving page
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(widget);

    //Inner layout that displays the control panel and rendered heightmap
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(outer_layout[0]);


    //Place the path generation control information
    let _control_panel_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(15), Constraint::Percentage(15) ,Constraint::Percentage(15),Constraint::Percentage(15)])
        .split(main_layout[0]);





    //Heightmap render block (different to the map generation so slightly rewritten)



    //Place the CLI
    ui_shared::render_CLI(app, frame, outer_layout[1]);


}

///Render the path generation control panel
fn render_control_panel(_app: &mut App, _frame : &mut Frame, _widget : Rect){


    //Top control (i.e. heightmaps to compare)


    




}
 
