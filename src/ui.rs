use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::app::{App, DeformType};

pub fn render(app: &mut App, frame: &mut Frame) {
    main_page(app, frame)
}

///The main page of the heightmap generator
/// Consists of 4 widgets - Title, Control Info, Heightmap Renderer, CLI
fn main_page(app: &mut App, frame: &mut Frame) {
    //Define the main window layout
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(85),
            Constraint::Percentage(10),
        ])
        .split(frame.area());

    //Inner layout that displays the control panel and rendered heightmap
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(outer_layout[1]);

    //Control panel layout
    let cntrl_panel_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main_layout[0]);

    //Render the title widget
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "Heightmap Generator - Version 0.01",
            Style::new().bold(),
        )))
        .alignment(Alignment::Center)
        .block(Block::new().borders(Borders::ALL)),
        outer_layout[0],
    );

    //Render the standard control panel
    let cntrl_panel_text = vec![
        Line::from(Span::styled("Shape positioning", Style::new().bold())),
        Line::from(format!(
            "Pos X: {}       Pos Y: {}",
            app.deform_center[0], app.deform_center[1]
        )),
        Line::from(format!(" Rotation (degs): {}", app.deform_rotation)),
        Line::from(format!("Thickness (mm): {}", app.deform_thickness)),
    ];

    frame.render_widget(
        Paragraph::new(cntrl_panel_text)
            .block(Block::new().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        cntrl_panel_layout[0],
    );

    //Render the deformaiton control panel
    match app.deform_type {
        DeformType::NONE => none_cntrl_panel(frame, cntrl_panel_layout[1]),
        DeformType::LINE | DeformType::CIRCLE | DeformType::RECTANGLE => {
            shape_cntrl_panel(app, frame, cntrl_panel_layout[1])
        }
    }

    //Render the heightmap
    if app.hmap_loaded{

    }else{
        frame.render_widget(
        Paragraph::new(Line::from(Span::styled("Load a heightmap", Style::new().bold())))
            .block(Block::new().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        main_layout[1],
    );
    }

    //CLI text
    let err_msg = if app.curr_error.is_empty() {
        Line::from(Span::styled(
            "Error status: All good B)",
            Style::new().green().italic(),
        ))
    } else {
        Line::from(Span::styled(
            format!("Error status: {}", app.curr_error),
            Style::new().red().bold(),
        ))
    };

    let cli_text = vec![err_msg, Line::from(format!(">{}_", app.curr_input))];

    //Render the CLI section
    frame.render_widget(
        Paragraph::new(cli_text)
            .alignment(Alignment::Left)
            .block(Block::new().borders(Borders::ALL)),
        outer_layout[2],
    );
}

///Control panel for telling the user to select a shape
fn none_cntrl_panel(frame: &mut Frame, widget: Rect) {
    let msg = Line::from("Select a shape using 'set shape [shape]'").centered();

    frame.render_widget(
        Paragraph::new(msg).block(Block::new().borders(Borders::ALL)),
        widget,
    );
}
///The control panel for creating a line
fn shape_cntrl_panel(app: &mut App, frame: &mut Frame, widget: Rect) {
    //Set the title
    let mut setting_lines = vec![
        Line::from(Span::styled(
            format!("Deformation: {}", app.deform_type.to_string()),
            Style::new().bold(),
        ))
        .centered(),
        Line::from(Span::styled(
            format!("Units are mm unless otherwise specified"),
            Style::new().bold(),
        ))
        .centered()
    ];

    //Go through each entry in the hashmap and add it as a line to the paragraph to be rendered
    for (setting, value) in &app.deform_settings {
        setting_lines.push(Line::from(format!("{}: {}", setting, value)))
    }

    frame.render_widget(
        Paragraph::new(setting_lines).block(Block::new().borders(Borders::ALL)),
        widget,
    )
}
