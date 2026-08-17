use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect, Offset},
    style::{Color, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Paragraph, Wrap, Tabs,
        canvas::{Canvas, Rectangle},
    },
};

use crate::app::{App, DeformType, HeightmapShape};

pub fn render(app: &mut App, frame: &mut Frame) {


    //Construct the page layout
    let layout = Layout::vertical([Constraint::Percentage(2), Constraint::Percentage(98)]);
    let [top, main] = frame.area().layout(&layout);

    //Render the tab titles
    let tabs = Tabs::new(vec!["Heightmap deformation", "Path generation"])
        .style(Color::White)
        .highlight_style(Style::default().magenta().on_black().bold())
        .select(app.tab_no)
        .divider(symbols::line::HEAVY_TRIPLE_DASH_VERTICAL)
        .padding(" ", " ");
    frame.render_widget(tabs, top);

    //Render the page based on the tab selection
    if app.tab_no == 0{
        main_page(app, frame,main)
    }
}

///The main page of the heightmap generator
/// Consists of 4 widgets - Title, Control Info, Heightmap Renderer, CLI
fn main_page(app: &mut App, frame: &mut Frame, area: Rect) {
    //Define the main window layout
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(area);

    //Inner layout that displays the control panel and rendered heightmap
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(outer_layout[0]);

    //Control panel layout
    let cntrl_panel_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main_layout[0]);


    //Determine the overlay line
    let overlay_line = if app.overlay_on {
        Line::from("Display overlay: [X]")
    } else {
        Line::from("Display overlay: [ ]")
    };

    let auto_gen_line = if app.auto_generate {
        Line::from("Auto generate: [X]")
    } else {
        Line::from("Auto generate: [ ]")
    };

    //Render the standard control panel
    let cntrl_panel_text = vec![
        Line::from(format!(
            "Pos X: {}       Pos Y: {}",
            app.core_settings.deform_center[0], app.core_settings.deform_center[1]
        )),
        Line::from(format!(
            " Rotation (degs): {}",
            app.core_settings.deform_rotation
        )),
        Line::from(format!(
            "Thickness (mm): {}",
            app.core_settings.deform_thickness
        )),
        Line::from(format!("Depth (mm): {}", app.core_settings.deform_depth)),
        overlay_line,
        auto_gen_line,
    ];

    frame.render_widget(
        Paragraph::new(cntrl_panel_text)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title(Line::from(Span::styled(
                        "Shape positioning",
                        Style::new().bold(),
                    ))),
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        cntrl_panel_layout[0],
    );

    //Render the deformation control panel
    match app.core_settings.deform_type {
        DeformType::NONE => none_cntrl_panel(frame, cntrl_panel_layout[1]),
        DeformType::LINE | DeformType::CIRCLE | DeformType::RECTANGLE => {
            shape_cntrl_panel(app, frame, cntrl_panel_layout[1])
        }
    }

    //Render the heightmap
    if app.hmap_loaded {
        render_heightmap(app, frame, main_layout[1])
    } else {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "Load a heightmap",
                Style::new().bold(),
            )))
            .block(Block::new().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
            main_layout[1],
        );
    }

    render_CLI(app, frame, outer_layout[1])
   
}

///Control panel for telling the user to select a shape
fn none_cntrl_panel(frame: &mut Frame, widget: Rect) {
    let msg = Line::from("Select a shape using 'set shape [shape]'").centered();

    frame.render_widget(
        Paragraph::new(msg).block(Block::new().borders(Borders::ALL).title(Line::from(
            Span::styled("Shape settings", Style::new().bold()),
        ))),
        widget,
    );
}
///The control panel for creating a line
fn shape_cntrl_panel(app: &mut App, frame: &mut Frame, widget: Rect) {
    //Set the title
    let mut setting_lines = vec![
        Line::from(Span::styled(
            format!("Deformation: {}", app.core_settings.deform_type.to_string()),
            Style::new().bold(),
        ))
        .centered(),
        Line::from(Span::styled(
            format!("Units are mm unless otherwise specified"),
            Style::new().bold(),
        ))
        .centered(),
    ];

    //Go through each entry in the hashmap and add it as a line to the paragraph to be rendered
    for (setting, value) in &app.deform_settings {
        setting_lines.push(Line::from(format!("{}: {}", setting, value)))
    }

    frame.render_widget(
        Paragraph::new(setting_lines).block(Block::new().borders(Borders::ALL).title(Line::from(
            Span::styled("Shape settings", Style::new().bold()),
        ))),
        widget,
    )
}

///Render the user CLI
fn render_CLI(app: &mut App, frame: &mut Frame, widget : Rect){
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
        widget,
    );
}

///Render the current heightmap onto a canvas
fn render_heightmap(app: &mut App, frame: &mut Frame, widget: Rect) {
    //Create the heightmap layout widget
    let data_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(6), Constraint::Percentage(94)])
        .split(widget);

    //Display the heightmap data in mm
    frame.render_widget(
        Paragraph::new(Line::from(format!(
            "Stats: Min depth:{}mm Max depth:{}mm  Pixel height:{} Pixel Width:{}",
            app.hmap_min,
            app.hmap_max,
            app.loaded_hmap.height() + 1,
            app.loaded_hmap.width() + 1
        )))
        .block(Block::new().borders(Borders::ALL)),
        data_layout[0],
    );

    //Display the precreated points
    //This method of pre-calculating saves a lot of energy when refreshing the frame
    if app.overlay_on {
        let mut canvas = Canvas::default()
            .block(Block::bordered().title("Current heightmap"))
            .x_bounds([0.0, app.loaded_hmap.width() as f64])
            .y_bounds([0.0, app.loaded_hmap.height() as f64])
            .marker(symbols::Marker::HalfBlock) //Half block displays the resolution the best
            .paint(|ctx| {
                ctx.draw(&HeightmapShape {
                    cells: &app.hmap_cells,
                });
                ctx.draw(&HeightmapShape {
                    cells: &app.generated_cells,
                })
            });

        //Create the canvas
        frame.render_widget(canvas, data_layout[1])
    } else {
        let mut canvas = Canvas::default()
            .block(Block::bordered().title("Current heightmap"))
            .x_bounds([0.0, app.loaded_hmap.width() as f64])
            .y_bounds([0.0, app.loaded_hmap.height() as f64])
            .marker(symbols::Marker::HalfBlock) //Half block displays the resolution the best
            .paint(|ctx| {
                ctx.draw(&HeightmapShape {
                    cells: &app.hmap_cells,
                });
            });

        //Create the canvas
        frame.render_widget(canvas, data_layout[1])
    }
}
