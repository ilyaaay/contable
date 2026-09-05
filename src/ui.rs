use crate::docker::DockerData;
use bollard::plugin::ImageSummary;
use ratatui::{
    DefaultTerminal, Frame, crossterm,
    layout::{Constraint, Direction, Layout},
    text::{Line, Span, Text},
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

#[derive(Default)]
struct App {
    state: AppState,
}

impl App {
    fn new() -> Self {
        Self {
            state: AppState::default(),
        }
    }
}

#[derive(Default)]
enum AppState {
    #[default]
    Active,
    Finish,
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

    let images = docker_data.images.to_string();

    frame.render_widget(
        Paragraph::new(images).block(Block::new().title("Images").borders(Borders::ALL)),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new("1").block(Block::new().title("Containers").borders(Borders::ALL)),
        layout[1],
    );
    frame.render_widget(
        Paragraph::new("2").block(Block::new().borders(Borders::ALL)),
        layout[2],
    );
}
