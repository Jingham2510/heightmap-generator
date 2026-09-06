/*
Shared UI widgets
*/


use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{
        Block, Borders, Paragraph,
    },
};

use crate::app::App;



///Render the user CLI
pub fn render_cmd_line(app: &mut App, frame: &mut Frame, widget : Rect){
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
