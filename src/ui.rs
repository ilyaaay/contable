use crate::docker::DockerData;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::read,
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};
use std::io;

pub fn app(terminal: &mut DefaultTerminal, docker_data: &DockerData) -> io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, docker_data))?;

        if let Some(key) = read()?.as_key_event()
            && key.code.is_char('q')
        {
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
        let lines = docker_data
            .images
            .get_images()
            .into_iter()
            .map(Line::from)
            .collect::<Vec<_>>();

        let block = Block::new().title("Images").borders(Borders::ALL);

        frame.render_widget(Paragraph::new(lines).block(block), frame.area());
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
