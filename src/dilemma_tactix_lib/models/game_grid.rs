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

use prettytable::{
    format::Alignment,
    Cell,
    Row,
    Table,
};

use crate::{
    Choice,
    GameOptions,
    NumberPair,
};

/// A representation of the game board.
///
/// The `GameGrid` struct is a representation of the game board. It encapsulates
/// the various values that are used to represent the game board, including the
/// Players Aleph and Beth, the four possible ways the two players can make
/// their choices, and the corresponding scores.
///
/// The `GameGrid` struct also contains the `GameOptions` struct, which contains
/// the various options that can be used to configure the game.
///
/// # Examples
///
/// ```
/// use dilemma_tactix_lib::{
///     Choice,
///     GameGrid,
///     GameOptions,
/// };
///
/// let game_options = GameOptions::builder("customized").build();
///
/// let game_grid = GameGrid::new(game_options);
///
/// game_grid.show_grid();
/// ```
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct GameGrid {
    pub game_options: GameOptions,
}

impl GameGrid {
    /// Creates a new `GameGrid` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     Choice,
    ///     GameGrid,
    ///     GameOptions,
    /// };
    ///
    /// let game_options = GameOptions::builder("customized").build();
    ///
    /// let game_grid = GameGrid::new(game_options);
    /// ```
    ///
    /// # Returns
    ///
    /// A new `GameGrid` instance.
    ///
    /// # See Also
    ///
    /// * [`GameOptions`](struct.GameOptions.html)
    /// * [`GameOptionsBuilder`](struct.GameOptionsBuilder.html)
    /// * [`GameOptions::new()`](struct.GameOptions.html#method.new)
    /// * [`GameOptions::default()`](struct.GameOptions.html#impl-Default)
    /// * [`GameOptions::builder()`](struct.GameOptions.html#method.builder)
    /// * [`GameOptionsBuilder::build()`](struct.GameOptionsBuilder.html#method.build)
    #[must_use]
    pub const fn new(game_options: GameOptions) -> Self {
        Self { game_options }
    }

    /// Format the `GameGrid` into a `Table`
    ///
    /// # Returns
    ///
    /// A `Table` representation of the `GameGrid`.
    ///
    /// # See Also
    ///
    /// * [`GameGrid::show_grid()`](struct.GameGrid.html#method.show_grid)
    /// * [`Table`](https://docs.rs/prettytable/0.8.0/prettytable/struct.Table.html)
    #[must_use]
    pub fn make_grid(&self) -> Table {
        let mut table = Table::new();

        table.add_row(Row::new(vec![
            Cell::new(""),
            Cell::new_align("Beth", Alignment::CENTER).with_hspan(2),
        ]));

        table.add_row(Row::new(vec![
            Cell::new("Aleph"),
            Cell::new(self.game_options.choice_atlantis()),
            Cell::new(self.game_options.choice_olympus()),
        ]));

        table.add_row(Row::new(vec![
            Cell::new(self.game_options.choice_atlantis()),
            Cell::new(self.game_options.atlantis_atlantis().to_string().as_str()),
            Cell::new(self.game_options.atlantis_olympus().to_string().as_str()),
        ]));

        table.add_row(Row::new(vec![
            Cell::new(self.game_options.choice_olympus()),
            Cell::new(self.game_options.olympus_atlantis().to_string().as_str()),
            Cell::new(self.game_options.olympus_olympus().to_string().as_str()),
        ]));

        table
    }

    ///  Display the `GameGrid` in the terminal.
    ///
    /// # Examples
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     Choice,
    ///     GameGrid,
    ///     GameOptions,
    /// };
    ///
    /// let game_options = GameOptions::builder("customized").build();
    ///
    /// let game_grid = GameGrid::new(game_options);
    ///
    /// game_grid.show_grid();
    /// ```
    ///
    /// # See Also
    ///
    /// * [`GameGrid::make_grid()`](struct.GameGrid.html#method.make_grid)
    /// * [`Table::printstd()`](https://docs.rs/prettytable/0.8.0/prettytable/struct.Table.html#method.printstd)
    /// * [`Table::to_string()`](https://docs.rs/prettytable/0.8.0/prettytable/struct.Table.html#method.to_string)
    pub fn show_grid(&self) {
        self.make_grid().printstd();
    }

    /// Return the score for the given choices.
    ///
    /// # Arguments
    ///
    /// * `aleph_choice` - The choice made by Player 1.
    /// * `beth_choice` - The choice made by Player 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     Choice,
    ///     GameGrid,
    ///     GameOptions,
    /// };
    ///
    /// let game_options = GameOptions::builder("customized").build();
    ///
    /// let game_grid = GameGrid::new(game_options);
    ///
    /// let result = game_grid.return_score(Choice::Atlantis, Choice::Atlantis);
    /// ```
    ///
    /// # Returns
    ///
    /// A `NumberPair` containing the scores for the given choices.
    ///
    /// # See Also
    ///
    /// * [`NumberPair`](struct.NumberPair.html)
    /// * [`GameOptions`](struct.GameOptions.html)
    #[must_use]
    pub const fn return_score(&self, aleph_choice: Choice, beth_choice: Choice) -> NumberPair {
        match (aleph_choice, beth_choice) {
            (Choice::Atlantis, Choice::Atlantis) => self.game_options.atlantis_atlantis(),
            (Choice::Atlantis, Choice::Olympus) => self.game_options.atlantis_olympus(),
            (Choice::Olympus, Choice::Atlantis) => self.game_options.olympus_atlantis(),
            (Choice::Olympus, Choice::Olympus) => self.game_options.olympus_olympus(),
        }
    }
}
