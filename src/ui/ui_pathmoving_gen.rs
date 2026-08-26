///A collection of widgets used in the 
use ratatui::{
    Frame, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::Style, symbols, text::{Line, Span}, widgets::{Block, Borders, Paragraph, Wrap, canvas::Canvas},
};

use crate::app::{App, HeightmapShape, path_gen_info};
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


   
    render_control_panel(app, frame, main_layout[0]);


    //Render path generation block
    render_path_gen_image(app, frame,main_layout[1]);
        


    //Place the CLI
    ui_shared::render_CLI(app, frame, outer_layout[1]);


}

///Render the path generation control panel
fn render_control_panel(app: &mut App, frame : &mut Frame, widget : Rect){

     //Create the path generation control layout
     //TODO: can rearrange fill priority based on textual requirements
    let control_panel_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10),Constraint::Percentage(5), Constraint::Fill(1) ,Constraint::Percentage(8),Constraint::Fill(1)])
        .split(widget);


    //Top control (i.e. heightmaps to compare)

    let hmap_lines : Vec<Line> = vec![
                                Line::from(format!("Current map: {}", app.path_gen_info.current_map_fp)), 
                                Line::from(format!("Target map: {}", app.path_gen_info.target_map_fp))];



     frame.render_widget(
        Paragraph::new(hmap_lines)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title(Line::from(Span::styled(
                        "Loaded terrains",
                        Style::new().bold(),
                    ))),
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        control_panel_layout[0],
    );


    let detect_lines : Vec<Line> = vec![
        Line::from(format!("Detection mode: {}", app.path_gen_info.detect_mode))
    ];


    //Detection setting display
    frame.render_widget(
        Paragraph::new(detect_lines)
            .block(
                Block::new()
                    .borders(Borders::ALL)
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        control_panel_layout[1],
    );

    //Detection settings set
   render_detect_settings(app, frame, control_panel_layout[2]);

    let path_lines : Vec<Line> = vec![
        Line::from(format!("Path generation mode: {}", app.path_gen_info.path_mode))
    ];


    //Detection setting display
    frame.render_widget(
        Paragraph::new(path_lines)
            .block(
                Block::new()
                    .borders(Borders::ALL)
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        control_panel_layout[3],
    );

    //Detection settings set
    frame.render_widget(
        Paragraph::new("Generation settings")
            .block(
                Block::new()
                .borders(Borders::ALL)
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        control_panel_layout[4],
    );

}

///Render the current detection settings
fn render_detect_settings(app: &mut App, frame:&mut Frame, widget : Rect){

    //Construct the detection settings list
    let mut setting_lines : Vec<Line> = vec![Line::from("Detection settings")];

    for (setting, value) in &app.path_gen_info.detect_info{
        setting_lines.push(Line::from(format!("{}: {}", setting, value)))
    }


     frame.render_widget(
        Paragraph::new(setting_lines)
            .block(
                Block::new()
                .borders(Borders::ALL)
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        widget,
    );
}


///Render the path generation imaging
fn render_path_gen_image(app: &mut App, frame : &mut Frame, widget : Rect){


    if !app.path_gen_info.diff_map_generated{
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "No difference map generated",
                Style::new().bold(),
            )))
            .block(Block::bordered().title("Difference map"))
            .centered(),
            widget);

    }else{
        let canvas = Canvas::default()
            .block(Block::bordered().title("Difference heightmap"))
            .x_bounds([0.0, app.path_gen_info.difference_width as f64])
            .y_bounds([0.0, app.path_gen_info.difference_height as f64])
            .marker(symbols::Marker::HalfBlock) //Half block displays the resolution the best
            .paint(|ctx| {
                ctx.draw(&HeightmapShape{
                    cells: &app.path_gen_info.diff_map_cells
                });
            });

        //Create the canvas
        frame.render_widget(canvas, widget)
    }




}
 
