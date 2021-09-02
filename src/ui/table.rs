use std::{error::Error, io};

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    Terminal,
    text::Span,
    text::Spans,
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::util::event::{Event, Events};

pub fn draw() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    loop {
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
            .style(Style::default().fg(Color::White))
            .header(
                Row::new(vec!["Col1", "Col2", "Col3"])
                    .style(Style::default().fg(Color::Yellow))
                    .bottom_margin(1)
            )
            .widths(&[Constraint::Length(5), Constraint::Length(5), Constraint::Length(10)])
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

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') => {
                    break;
                }
                _ => {}
            }
        };
    }

    Ok(())
}