// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * Copyright (c) 2023-2024
// *
// * This project is dual-licensed under the MIT and Apache licenses.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** APACHE 2.0 LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Licensed under the Apache License, Version 2.0 (the "License");
// * you may not use this file except in compliance with the License.
// * You may obtain a copy of the License at
// *
// * http://www.apache.org/licenses/LICENSE-2.0
// *
// * Unless required by applicable law or agreed to in writing, software
// * distributed under the License is distributed on an "AS IS" BASIS,
// * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// * See the License for the specific language governing permissions and
// * limitations under the License.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * * MIT LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Permission is hereby granted, free of charge, to any person obtaining a copy
// * of this software and associated documentation files (the "Software"), to deal
// * in the Software without restriction, including without limitation the rights
// * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// * copies of the Software, and to permit persons to whom the Software is
// * furnished to do so, subject to the following conditions:
// *
// * The above copyright notice and this permission notice shall be included in all
// * copies or substantial portions of the Software.
// *
// * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// * SOFTWARE.
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

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
    let size = frame.size();

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
