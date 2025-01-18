// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use ratatui::{
    prelude::*,
    widgets::{
        Block,
        BorderType,
        Borders,
        Padding,
        Paragraph,
    },
};

pub use crate::app::App;

fn render_header(frame: &mut Frame, rect: Rect) {
    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::new(1, 1, 1, 0));
    let header = Paragraph::new("Tactix")
        .style(Style::new().white().on_red().bold())
        .centered()
        .block(header_block);

    frame.render_widget(header, rect);
}

fn render_footer(frame: &mut Frame, rect: Rect) {
    let footer = Paragraph::new("Press 'Q' to quit".to_string())
        .block(
            Block::default()
                .title("")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::new().white().on_black())
        .centered();

    frame.render_widget(footer, rect);
}

pub fn ui(frame: &mut Frame, app: &App) {
    let size = frame.area();

    let screen_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .flex(layout::Flex::Center)
        .split(size);

    render_header(frame, screen_layout[0]);

    let main_content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 5),
            Constraint::Ratio(3, 5),
            Constraint::Ratio(1, 5),
        ])
        .flex(layout::Flex::Center)
        .split(screen_layout[1]);

    let game_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20); 5])
        .split(main_content_layout[1]);

    let beth_header_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 5),
            Constraint::Ratio(1, 5),
            Constraint::Ratio(2, 5),
            Constraint::Ratio(1, 5),
        ])
        .split(game_rows[1]);

    let aleph_header_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20); 5])
        .split(game_rows[2]);

    let atlantis_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20); 5])
        .split(game_rows[3]);

    let olympus_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20); 5])
        .split(game_rows[4]);

    let beth_text = Paragraph::new("Player Beth")
        .block(Block::default().title("").borders(Borders::ALL))
        .style(Style::new().white().on_black())
        .alignment(Alignment::Center);

    frame.render_widget(beth_text, beth_header_row[2]);

    let aleph_text = Paragraph::new("Player Aleph")
        .block(Block::default().title("").borders(Borders::ALL))
        .style(Style::new().white().on_black())
        .alignment(Alignment::Center);

    frame.render_widget(aleph_text, aleph_header_row[1]);

    let atlantis_text = Paragraph::new(app.game_grid.game_options.choice_atlantis().to_string())
        .block(Block::default().title("").borders(Borders::ALL))
        .style(Style::new().white().on_black())
        .alignment(Alignment::Center);

    frame.render_widget(atlantis_text.clone(), aleph_header_row[2]);
    frame.render_widget(atlantis_text.clone(), atlantis_row[1]);

    let atlantis_atlantis_score =
        Paragraph::new(app.game_grid.game_options.atlantis_atlantis().to_string())
            .block(Block::default().title("").borders(Borders::ALL))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Center);

    frame.render_widget(atlantis_atlantis_score, atlantis_row[2]);

    let atlantis_olympus_score =
        Paragraph::new(app.game_grid.game_options.atlantis_olympus().to_string())
            .block(Block::default().title("").borders(Borders::ALL))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Center);

    frame.render_widget(atlantis_olympus_score, atlantis_row[3]);

    let olympus_text = Paragraph::new(app.game_grid.game_options.choice_olympus().to_string())
        .block(Block::default().title("").borders(Borders::ALL))
        .style(Style::new().white().on_black())
        .alignment(Alignment::Center);

    frame.render_widget(olympus_text.clone(), aleph_header_row[3]);
    frame.render_widget(olympus_text.clone(), olympus_row[1]);

    let olympus_atlantis_score =
        Paragraph::new(app.game_grid.game_options.olympus_atlantis().to_string())
            .block(Block::default().title("").borders(Borders::ALL))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Center);

    frame.render_widget(olympus_atlantis_score, olympus_row[2]);

    let olympus_olympus_score =
        Paragraph::new(app.game_grid.game_options.olympus_olympus().to_string())
            .block(Block::default().title("").borders(Borders::ALL))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Center);

    frame.render_widget(olympus_olympus_score, olympus_row[3]);

    render_footer(frame, screen_layout[2]);
}
