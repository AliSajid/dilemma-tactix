// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * Copyright (c) 2023
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
// *     http://www.apache.org/licenses/LICENSE-2.0
// *
// * Unless required by applicable law or agreed to in writing, software
// * distributed under the License is distributed on an "AS IS" BASIS,
// * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// * See the License for the specific language governing permissions and
// * limitations under the License.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** MIT LICENSE
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

use std::fmt::Display;

use crate::GameOptions;
use crate::NumberPair;

/// A struct to encapsulate a single game grid
///
/// This struct abstracts the concept of one 2x2 Pirsoner's Dilemma
/// game grid. It contains the game options, and the scores for each
/// of the four possible outcomes.
///
/// This struct allows us to generate a randomized game grid, or a specific
/// game grid. For this reason, the options for the scores and the choices
/// have been abstracted to the [`GameOptions`](crate::GameOptions) struct.
///
/// # Example
///
/// ## Specific Game Grid
///
/// ```
/// use dilemma_tactix_lib::{GameGrid, NumberPair};
///
/// let game_grid = GameGrid::new(
///    10,
///   1,
///  "A".to_string(),
/// "B".to_string(),
/// NumberPair::new(1, 2),
/// NumberPair::new(3, 4),
/// NumberPair::new(5, 6),
/// NumberPair::new(7, 8),
/// );
///
/// assert_eq!(game_grid.max_value(), 10);
/// assert_eq!(game_grid.min_value(), 1);
/// assert_eq!(game_grid.choice_aleph(), "A");
/// assert_eq!(game_grid.choice_bey(), "B");
/// assert_eq!(game_grid.score_aa(), NumberPair::new(1, 2));
/// assert_eq!(game_grid.score_ab(), NumberPair::new(3, 4));
/// assert_eq!(game_grid.score_ba(), NumberPair::new(5, 6));
/// assert_eq!(game_grid.score_bb(), NumberPair::new(7, 8));
/// ```
///
/// ## Default Game Grid
///
/// ```
/// use dilemma_tactix_lib::GameGrid;
/// use dilemma_tactix_lib::ChoiceNameOptions;
///
/// let choice_name_options = ChoiceNameOptions::new();
/// let game_grid = GameGrid::default();
/// assert!(game_grid.max_value() <= 10);
/// assert!(game_grid.min_value() <= 1);
/// assert_ne!(game_grid.choice_aleph(), game_grid.choice_bey());
/// assert!(choice_name_options.choice_aleph_options.contains(&game_grid.choice_aleph()));
/// assert!(choice_name_options.choice_bey_options.contains(&game_grid.choice_bey()));
///
/// ```
pub struct GameGrid {
    game_options: GameOptions,
    score_aa: NumberPair,
    score_ab: NumberPair,
    score_ba: NumberPair,
    score_bb: NumberPair,
}

impl GameGrid {
    #[allow(clippy::too_many_arguments, clippy::similar_names)]
    #[must_use]
    pub fn new(
        max_value: u32,
        min_value: u32,
        choice_aleph: String,
        choice_bey: String,
        score_aa: NumberPair,
        score_ab: NumberPair,
        score_ba: NumberPair,
        score_bb: NumberPair,
    ) -> Self {
        let game_options = GameOptions::new(min_value, max_value, choice_aleph, choice_bey);
        Self {
            game_options,
            score_aa,
            score_ab,
            score_ba,
            score_bb,
        }
    }

    #[must_use]
    pub const fn max_value(&self) -> u32 {
        self.game_options.max_value()
    }

    #[must_use]
    pub const fn min_value(&self) -> u32 {
        self.game_options.min_value()
    }

    #[must_use]
    pub fn choice_aleph(&self) -> &str {
        self.game_options.choice_aleph()
    }

    #[must_use]
    pub fn choice_bey(&self) -> &str {
        self.game_options.choice_bey()
    }

    #[must_use]
    pub const fn score_aa(&self) -> NumberPair {
        self.score_aa
    }

    #[must_use]
    pub const fn score_ab(&self) -> NumberPair {
        self.score_ab
    }

    #[must_use]
    pub const fn score_ba(&self) -> NumberPair {
        self.score_ba
    }

    #[must_use]
    pub const fn score_bb(&self) -> NumberPair {
        self.score_bb
    }
}

impl Display for GameGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Game Grid with Following Options:\n{}\n",
            self.game_options,
        )
    }
}

impl Default for GameGrid {
    fn default() -> Self {
        let game_options = GameOptions::default();
        let min_value = game_options.min_value();
        let max_value = game_options.max_value();
        Self {
            game_options,
            score_aa: NumberPair::random(min_value, max_value),
            score_ab: NumberPair::random(min_value, max_value),
            score_ba: NumberPair::random(min_value, max_value),
            score_bb: NumberPair::random(min_value, max_value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ChoiceNameOptions;

    use super::*;

    #[test]
    fn test_game_grid_display() {
        let game_grid = GameGrid::new(
            10,
            1,
            "A".to_string(),
            "B".to_string(),
            NumberPair::new(1, 2),
            NumberPair::new(3, 4),
            NumberPair::new(5, 6),
            NumberPair::new(7, 8),
        );
        assert_eq!(
            format!("{}", game_grid),
            "Game Grid with Following Options:\nmin_value: 1, max_value: 10, choice_aleph: A, choice_bey: B\n"
        );
    }

    #[test]
    fn test_game_grid_default() {
        let game_grid = GameGrid::default();
        let choice_name_options = ChoiceNameOptions::new();
        assert_eq!(game_grid.min_value(), 1);
        assert_eq!(game_grid.max_value(), 10);
        assert_ne!(game_grid.choice_aleph(), game_grid.choice_bey());
        assert!(choice_name_options
            .choice_aleph_options
            .contains(&game_grid.choice_aleph()));
        assert!(choice_name_options
            .choice_bey_options
            .contains(&game_grid.choice_bey()));
    }

    #[test]
    fn test_game_grid_new() {
        let game_grid = GameGrid::new(
            10,
            1,
            "A".to_string(),
            "B".to_string(),
            NumberPair::new(1, 2),
            NumberPair::new(3, 4),
            NumberPair::new(5, 6),
            NumberPair::new(7, 8),
        );
        assert_eq!(game_grid.max_value(), 10);
        assert_eq!(game_grid.min_value(), 1);
        assert_eq!(game_grid.choice_aleph(), "A");
        assert_eq!(game_grid.choice_bey(), "B");
    }

    #[test]
    fn test_game_grid_score_aa() {
        let game_grid = GameGrid::new(
            10,
            1,
            "A".to_string(),
            "B".to_string(),
            NumberPair::new(1, 2),
            NumberPair::new(3, 4),
            NumberPair::new(5, 6),
            NumberPair::new(7, 8),
        );
        assert_eq!(game_grid.score_aa(), NumberPair::new(1, 2));
    }

    #[test]
    fn test_game_grid_score_ab() {
        let game_grid = GameGrid::new(
            10,
            1,
            "A".to_string(),
            "B".to_string(),
            NumberPair::new(1, 2),
            NumberPair::new(3, 4),
            NumberPair::new(5, 6),
            NumberPair::new(7, 8),
        );
        assert_eq!(game_grid.score_ab(), NumberPair::new(3, 4));
    }
}