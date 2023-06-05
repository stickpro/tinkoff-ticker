use std::io;
use tui::{layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, widgets::{Table, Row, Cell }};
use tui::widgets::{Block};
use tui::{backend::Backend, Terminal};
use tui::text::{ Span, Spans};

use crate::tinkoff_api::models::PortfolioPosition;

pub fn draw<B: Backend>(
    terminal: &mut Terminal<B>,
    _positions: &[PortfolioPosition],
) -> Result<(), io::Error> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());
        //let block = Block::default().title("Portfolio").borders(Borders::ALL);

        let table = Table::new(vec![
            Row::new(vec!["Row11", "Row12", "Row13"]),
            Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::default().fg(Color::Blue)),
            Row::new(vec![
                Cell::from("Row31"),
                Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                Cell::from(Spans::from(vec![
                    Span::raw("Row"),
                    Span::styled("33", Style::default().fg(Color::Green))
                ])),
            ]),
            Row::new(vec![
                Cell::from("Row\n41"),
                Cell::from("Row\n42"),
                Cell::from("Row\n43"),
            ]).height(2),
        ])
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["Col1", "Col2", "Col3"])
                .style(Style::default().fg(Color::Yellow))
                .bottom_margin(1)
        )
        .block(Block::default().title("Table"))
        .widths(&[Constraint::Length(5), Constraint::Length(5), Constraint::Length(10)])
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

        f.render_widget(table, chunks[0]);

        // for item in positions {

        //     if let Some(ticker) = item.ticker {
        //         f.render_widget(ticker, chunks[0]);
        //     }
        // }
    })
}
