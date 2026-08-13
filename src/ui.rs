use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
    text::{Line, Span}
};

use crate::app::{App, DeformType};

pub fn render(app: &mut App, frame: &mut Frame) {
    /*
    frame.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Counter: {}
      ",
            app.counter
        ))
        .block(
            Block::default()
                .title("Counter App")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        frame.area(),
    )
    */

    main_page(app, frame)

}

///The main page of the heightmap generator
/// Consists of 4 widgets - Title, Control Info, Heightmap Renderer, CLI
fn main_page(app: &mut App, frame: &mut Frame){

    //Define the main window layout
    let outer_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(5),
        Constraint::Percentage(85),
        Constraint::Percentage(10)
    ])
    .split(frame.area());

    
    //Inner layout that displays the control panel and rendered heightmap
    let main_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(30),
        Constraint::Percentage(70)
    ]).split(outer_layout[1]);


    //Control panel layout
    let cntrl_panel_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(30),
        Constraint::Percentage(70)
    ]).split(main_layout[0]);

    //Render the title widget
    frame.render_widget(
            Paragraph::new(Line::from(Span::styled("Heightmap Generator - Version 0.01", Style::new().bold())))
            .alignment(Alignment::Center)
            .block(Block::new().borders(Borders::ALL)),
            outer_layout[0]);


    //Render the standard control panel
    let cntrl_panel_text = vec![
        Line::from(Span::styled("Shape positioning", Style::new().bold())),
        Line::from(format!("Pos X: {}       Pos Y: {}", app.deform_center[0], app.deform_center[1])),
        Line::from(format!(" Rotation (degs): {}", app.deform_rotation))
        ];

    frame.render_widget(
        Paragraph::new(cntrl_panel_text).block(Block::new().borders(Borders::ALL)).alignment(Alignment::Center).wrap(Wrap { trim: true}),
    cntrl_panel_layout[0]);

    //Render the deformaiton control panel
    match app.deform_type{
        DeformType::NONE => {none_cntrl_panel(frame, cntrl_panel_layout[1])},
        DeformType::LINE => {line_cntrl_panel(app, frame, cntrl_panel_layout[1])},
        DeformType::CIRCLE => {todo!()},
        DeformType::SQUARE => {todo!()}
    }

    //Render the heightmap


    //CLI text
    let err_msg = if app.curr_error.is_empty(){
        Line::from(Span::styled("Error status: All good B)", Style::new().green().italic()))
    }else{
        Line::from(Span::styled(format!("Error status: {}", app.curr_error), Style::new().red().bold()))
    };

    let cli_text = vec![
        err_msg,
        Line::from(format!(">{}_", app.curr_input))
    ];

    //Render the CLI section
    frame.render_widget(
        Paragraph::new(cli_text)
        .alignment(Alignment::Left)
        .block(Block::new().borders(Borders::ALL)),
        outer_layout[2]
    );
}

///Control panel for telling the user to select a shape
fn none_cntrl_panel(frame: &mut Frame, widget: Rect){

    let msg = Line::from("Select a shape using 'set [shape]'")
        .centered();


    frame.render_widget(
        Paragraph::new(msg)
        .block(Block::new()
        .borders(Borders::ALL)), 
        widget
    );


}
///The control panel for creating a line
fn line_cntrl_panel(app: &mut App, frame: &mut Frame, widget: Rect){

    //frame.render_widget()


}