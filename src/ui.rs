use crate::docker::DockerData;
use ratatui::{
    DefaultTerminal, Frame, crossterm,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};
use std::io;

pub fn app(terminal: &mut DefaultTerminal, docker_data: &DockerData) -> io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, docker_data))?;

        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, docker_data: &DockerData) {
    let partitions_in_persents = [
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(33),
    ];

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(partitions_in_persents)
        .split(frame.area());

    {
        let images = docker_data
            .images
            .get_strings()
            .into_iter()
            .map(Line::from)
            .collect::<Vec<_>>();

        frame.render_widget(
            Paragraph::new(images).block(Block::new().title("Images").borders(Borders::ALL)),
            layout[0],
        );
    }

    frame.render_widget(
        Paragraph::new("1").block(Block::new().title("Containers").borders(Borders::ALL)),
        layout[1],
    );

    frame.render_widget(
        Paragraph::new("2").block(Block::new().borders(Borders::ALL)),
        layout[2],
    );
}
