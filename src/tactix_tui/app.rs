// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use dilemma_tactix_lib::{
    Choice,
    GameGrid,
};

#[allow(dead_code)]
pub struct App {
    pub game_grid: GameGrid,
    pub selected:  Option<Choice>,
}

impl App {
    pub const fn new(game_grid: GameGrid) -> Self {
        Self {
            game_grid,
            selected: None,
        }
    }
}
