// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{
    error::Error,
    io,
};

use crossterm::{
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode,
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use dilemma_tactix_lib::{
    GameGrid,
    GameOptions,
};
use ratatui::{
    backend::{
        Backend,
        CrosstermBackend,
    },
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::App,
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let game_options = GameOptions::builder("customized").build();
    let game = GameGrid::new(game_options);

    // create app and run it
    let mut app = App::new(game);
    let _res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match key.code {
                KeyCode::Char('q') => {
                    eprint!("'q' pressed; breaking loop");
                    break;
                }
                KeyCode::Esc => {
                    eprint!("ESC pressed; breaking loop");
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(false)
}
