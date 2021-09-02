use std::{error::Error, io};

use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    Terminal,
    text::Span,
    text::Spans,
    widgets::{Block, Borders, Cell, Row, Table},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let table = Table::new(vec![
        Row::new(vec![
            Cell::from("Row31"),
            Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
            Cell::from(Spans::from(vec![
                Span::raw("Row"),
                Span::styled("33", Style::default().fg(Color::Green))
            ])),
        ]),
    ])
        // You can set the style of the entire Table.
        .style(Style::default().fg(Color::White))
        // It has an optional header, which is simply a Row always visible at the top.
        .header(
            Row::new(vec!["Col1", "Col2", "Col3"])
                .style(Style::default().fg(Color::Yellow))
                // If you want some space between the header and the rest of the rows, you can always
                // specify some margin at the bottom.
                .bottom_margin(1)
        )
        // Columns widths are constrained in the same way as Layout...
        .widths(&[Constraint::Length(5), Constraint::Length(5), Constraint::Length(10)])
        // ...and they can be separated by a fixed spacing.
        .column_spacing(1);


    terminal.draw(|f| {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(5)
            .split(f.size());

        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);

        let size = f.size();
        // f.render_stateful_widget(table, rects[0], &mut state);
        f.render_widget(table, rects[0]);
        f.render_widget(block, size);
    })?;


    Ok(())
}