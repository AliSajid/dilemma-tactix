// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt::Display;

#[cfg(test)]
use crate::RANDOM_SEED;
use crate::{
    ChoiceNameOptions,
    GameOptionsBuilder,
    NumberPair,
};

/// This is a struct that holds the options for a game.
///
/// This struct is used to encapsulate the parameters to be used for generating
/// a single grid from the parameters that may be related to a tournament or a
/// series of grids.
///
/// The two parameters are:
///
/// * `min_value` - The minimum value for that can be assigned to a choice.
/// * `max_value` - The maximum value for that can be assigned to a choice.
///
/// # Example
///
/// ## Explicit Options
///
/// ```
/// use dilemma_tactix_lib::GameOptions;
///
/// let game_options = GameOptions::new(1, 10);
/// ```
///
/// ## Default Options
///
/// ```
/// use dilemma_tactix_lib::GameOptions;
///
/// let game_options = GameOptions::default();
/// ```
///
/// ## Builder
///
/// ```
/// use dilemma_tactix_lib::GameOptions;
///
/// let game_options = GameOptions::builder("customized").build();
/// ```
///
/// # Notes
///
/// The `GameOptions` struct implements the `Default` trait, and can be created
/// with the `default()` method.
///
/// # See Also
///
/// * [`GameOptions::new()`](#method.new)
/// * [`GameOptions::default()`](#method.default)
/// * [`GameOptions::builder()`](#method.builder)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameOptions {
    /// The label for the first choice that can be made
    pub choice_atlantis:   &'static str,
    /// The label for the second choice that can be made
    pub choice_olympus:    &'static str,
    /// Score for Aleph-Atlantis and Beth-Atlantis
    pub atlantis_atlantis: NumberPair,
    /// Score for Aleph-Atlantis and Beth-Olympus
    pub atlantis_olympus:  NumberPair,
    /// Score for Aleph-Olympus and Beth-Atlantis
    pub olympus_atlantis:  NumberPair,
    /// Score for Aleph-Olympus and Beth-Olympus
    pub olympus_olympus:   NumberPair,
}

impl GameOptions {
    /// Creates a new `GameOptions` struct.
    ///
    /// This function creates a new `GameOptions` struct with the given
    /// parameters.
    ///
    /// In the implementation, the new struct instance instantiates the
    /// `ChoiceNameOptions` struct, and then uses it to get a random pair of
    /// choices. It then uses the given `min_value` and `max_value` to generate
    /// random scores for the four possible combinations of outcomes.
    ///
    /// # Arguments
    ///
    /// * `min_value` - The minimum score for that can be assigned to a choice.
    /// * `max_value` - The maximum score for that can be assigned to a choice.
    ///
    /// # Panics
    ///
    /// Panics if `min_value` is greater than `max_value`.
    ///
    /// # See Also
    ///
    /// * [`GameOptions::default()`](#method.default)
    #[must_use]
    pub fn new(min_value: u32, max_value: u32) -> Self {
        #[cfg(test)]
        let (choice_atlantis, choice_olympus) =
            ChoiceNameOptions::get_random_pair_seeded(RANDOM_SEED.0);

        #[cfg(test)]
        let (atlantis_atlantis, atlantis_olympus, olympus_atlantis, olympus_olympus) = (
            NumberPair::random_seeded(min_value, max_value, RANDOM_SEED.0),
            NumberPair::random_seeded(min_value, max_value, RANDOM_SEED.1),
            NumberPair::random_seeded(min_value, max_value, RANDOM_SEED.2),
            NumberPair::random_seeded(min_value, max_value, RANDOM_SEED.3),
        );

        #[cfg(not(test))]
        let (choice_atlantis, choice_olympus) = ChoiceNameOptions::get_random_pair();

        #[cfg(not(test))]
        let (atlantis_atlantis, atlantis_olympus, olympus_atlantis, olympus_olympus) = (
            NumberPair::random(min_value, max_value),
            NumberPair::random(min_value, max_value),
            NumberPair::random(min_value, max_value),
            NumberPair::random(min_value, max_value),
        );

        Self {
            choice_atlantis,
            choice_olympus,
            atlantis_atlantis,
            atlantis_olympus,
            olympus_atlantis,
            olympus_olympus,
        }
    }

    /// Returns the value of `choice_atlantis`.
    ///
    /// This function returns the value of `choice_atlantis`.
    ///
    /// # Returns
    ///
    /// The value of `choice_atlantis`.
    ///
    /// # See Also
    ///
    /// * [`GameOptions::min_value()`](#method.min_value)
    /// * [`GameOptions::max_value()`](#method.max_value)
    /// * [`GameOptions::choice_olympus()`](#method.choice_olympus)
    #[must_use]
    pub const fn choice_atlantis(&self) -> &str {
        self.choice_atlantis
    }

    /// Returns the value of `choice_olympus`.
    ///
    /// This function returns the value of `choice_olympus`.
    ///
    /// # Returns
    ///
    /// The value of `choice_olympus`.
    ///
    /// # See Also
    ///
    /// * [`GameOptions::min_value()`](#method.min_value)
    /// * [`GameOptions::max_value()`](#method.max_value)
    /// * [`GameOptions::choice_atlantis()`](#method.choice_atlantis)
    #[must_use]
    pub const fn choice_olympus(&self) -> &str {
        self.choice_olympus
    }

    /// Returns the value of `atlantis_atlantis`.
    ///
    /// This function returns the value of `atlantis_atlantis`, which is the
    /// `NumberPair` containing the scores for the case when both Player Aleph
    /// and Player Beth choose the Atlantis strategy.
    ///
    /// # Returns
    ///
    /// The value of `atlantis_atlantis` as a
    /// [`NumberPair`](#struct.NumberPair).
    ///
    /// # See Also
    ///
    /// * [`GameOptions::atlantis_olympus()`](#method.atlantis_olympus)
    /// * [`GameOptions::olympus_atlantis()`](#method.olympus_atlantis)
    /// * [`GameOptions::olympus_olympus()`](#method.olympus_olympus)
    #[must_use]
    pub const fn atlantis_atlantis(&self) -> NumberPair {
        self.atlantis_atlantis
    }

    /// Returns the value of `atlantis_olympus`.
    ///
    /// This function returns the value of `atlantis_olympus`, which is the
    /// `NumberPair` containing the scores for the case when Player Aleph
    /// chooses the Atlantis strategy and Player Beth chooses the Olympus
    /// strategy.
    ///
    /// # Returns
    ///
    /// The value of `atlantis_olympus` as a [`NumberPair`](#struct.NumberPair).
    ///
    /// # See Also
    ///
    /// * [`GameOptions::atlantis_atlantis()`](#method.atlantis_atlantis)
    /// * [`GameOptions::olympus_atlantis()`](#method.olympus_atlantis)
    /// * [`GameOptions::olympus_olympus()`](#method.olympus_olympus)
    #[must_use]
    pub const fn atlantis_olympus(&self) -> NumberPair {
        self.atlantis_olympus
    }

    /// Returns the value of `olympus_atlantis`.
    ///
    /// This function returns the value of `olympus_atlantis`, which is the
    /// `NumberPair` containing the scores for the case when Player Aleph
    /// chooses the Olympus strategy and Player Beth chooses the Atlantis
    /// strategy.
    ///
    /// # Returns
    ///
    /// The value of `olympus_atlantis` as a [`NumberPair`](#struct.NumberPair).
    ///
    /// # See Also
    ///
    /// * [`GameOptions::atlantis_atlantis()`](#method.atlantis_atlantis)
    /// * [`GameOptions::atlantis_olympus()`](#method.atlantis_olympus)
    /// * [`GameOptions::olympus_olympus()`](#method.olympus_olympus)
    #[must_use]
    pub const fn olympus_atlantis(&self) -> NumberPair {
        self.olympus_atlantis
    }

    /// Returns the value of `olympus_olympus`.
    ///
    /// This function returns the value of `olympus_olympus`, which is the
    /// `NumberPair` containing the scores for the case when both Player Aleph
    /// and Player Beth choose the Olympus strategy.
    ///
    /// # Returns
    ///
    /// The value of `olympus_olympus` as a [`NumberPair`](#struct.NumberPair).
    ///
    /// # See Also
    ///
    /// * [`GameOptions::atlantis_atlantis()`](#method.atlantis_atlantis)
    /// * [`GameOptions::atlantis_olympus()`](#method.atlantis_olympus)
    /// * [`GameOptions::olympus_atlantis()`](#method.olympus_atlantis)
    #[must_use]
    pub const fn olympus_olympus(&self) -> NumberPair {
        self.olympus_olympus
    }

    /// Create a builder for a `GameOptions` struct.
    ///
    /// This function creates a builder for a `GameOptions` struct which allows
    /// for step-by-step building of individual game options objects, bypassing
    /// random generation of values if desired.
    ///
    /// # Arguments
    ///
    /// * `builder_type` - The type of builder to create. Valid values are `randomized`, `seeded`,
    ///   and `customized`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptions;
    ///
    /// let game_options = GameOptions::builder("customized");
    /// ```
    ///
    /// # Returns
    ///
    /// A new `GameOptionsBuilder` struct.
    ///
    /// # Notes
    ///
    /// Unlike the `default` method. a `default` builder is guaranteed to
    /// be the same each time it is called.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder`](#struct.GameOptionsBuilder)
    /// * [`GameOptions::new()`](#method.new)
    /// * [`GameOptions::default()`](#method.default)
    /// * [`GameOptionsBuilder::build()`](#method.build)
    #[must_use]
    pub fn builder(builder_type: &str) -> GameOptionsBuilder {
        match builder_type {
            "seeded" | "Seeded" => {
                GameOptionsBuilder::new(super::game_option_builder::GameOptionsBuilderTypes::Seeded)
            }
            "customized" | "Customized" => GameOptionsBuilder::new(
                super::game_option_builder::GameOptionsBuilderTypes::Customized,
            ),
            _ => GameOptionsBuilder::new(
                super::game_option_builder::GameOptionsBuilderTypes::Randomized,
            ),
        }
    }
}

impl Default for GameOptions {
    /// Creates a new `GameOptions` struct with default values.
    ///
    /// This function creates a new `GameOptions` struct with default values.
    ///
    /// The default values are:
    ///
    /// * `min_value` - 1
    /// * `max_value` - 10
    /// * `choice_atlantis` - "cooperate"
    /// * `choice_olympus` - "defect"
    ///
    /// # Returns
    ///
    /// A new `GameOptions` struct with default values for the parameters.
    ///
    /// # Notes
    ///
    /// Unlike the default options in the `builder()` method, the default
    /// options here are guaranteed to generate a new random grid each time.
    ///
    /// # See Also
    ///
    /// * [`GameOptions::new()`](#method.new)
    /// * [`GameOptions::builder()`](#method.builder)
    fn default() -> Self {
        Self::new(1, 10)
    }
}

impl Display for GameOptions {
    /// Implements the Display trait for `GameOptions`.
    ///
    /// This function implements the Display trait for `GameOptions`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "choice_atlantis: {}, choice_olympus: {}, atlantis_atlantis: {}, atlantis_olympus: \
             {}, olympus_atlantis: {}, olympus_olympus: {}",
            self.choice_atlantis,
            self.choice_olympus,
            self.atlantis_atlantis,
            self.atlantis_olympus,
            self.olympus_atlantis,
            self.olympus_olympus
        )
    }
}

#[cfg(test)]
mod tests {

    use rstest::{
        fixture,
        rstest,
    };

    use super::*;

    #[fixture]
    fn choice_atlantis_options() -> [&'static str; 17] {
        ChoiceNameOptions::choice_atlantis_options()
    }

    #[fixture]
    fn choice_olympus_options() -> [&'static str; 17] {
        ChoiceNameOptions::choice_olympus_options()
    }

    #[rstest]
    fn test_game_options_default(
        choice_atlantis_options: [&'static str; 17],
        choice_olympus_options: [&'static str; 17],
    ) {
        let game_options = GameOptions::default();

        assert_eq!(game_options.atlantis_atlantis(), NumberPair::new(6, 9));

        assert_eq!(game_options.atlantis_olympus(), NumberPair::new(3, 8));

        assert_eq!(game_options.olympus_atlantis(), NumberPair::new(6, 7));

        assert_eq!(game_options.olympus_olympus(), NumberPair::new(3, 6));

        assert_eq!(game_options.choice_atlantis(), "discrete");

        assert_eq!(game_options.choice_olympus(), "continuous");

        assert!(choice_atlantis_options.contains(&game_options.choice_atlantis()));

        assert!(choice_olympus_options.contains(&game_options.choice_olympus()));
    }

    #[test]
    fn test_game_options_new() {
        let game_options = GameOptions::new(1, 10);

        assert_eq!(game_options.atlantis_atlantis(), NumberPair::new(6, 9));

        assert_eq!(game_options.atlantis_olympus(), NumberPair::new(3, 8));

        assert_eq!(game_options.olympus_atlantis(), NumberPair::new(6, 7));

        assert_eq!(game_options.olympus_olympus(), NumberPair::new(3, 6));

        assert_eq!(game_options.choice_atlantis(), "discrete");

        assert_eq!(game_options.choice_olympus(), "continuous");
    }

    #[test]
    fn test_game_options_display() {
        let game_options = GameOptions::new(1, 10);

        assert_eq!(
            format!("{}", game_options),
            "choice_atlantis: discrete, choice_olympus: continuous, atlantis_atlantis: (6, 9), \
             atlantis_olympus: (3, 8), olympus_atlantis: (6, 7), olympus_olympus: (3, 6)"
        );
    }

    #[test]
    fn test_builder_randomized() {
        let builder = GameOptions::builder("randomized");

        assert!(builder.choice_atlantis.is_none());

        assert!(builder.choice_olympus.is_none());

        assert!(builder.atlantis_atlantis.is_none());

        assert!(builder.atlantis_olympus.is_none());

        assert!(builder.olympus_atlantis.is_none());

        assert!(builder.olympus_olympus.is_none());
    }

    #[test]
    fn test_builder_seeded() {
        let builder = GameOptions::builder("seeded");

        assert!(builder.choice_atlantis.is_none());

        assert!(builder.choice_olympus.is_none());

        assert!(builder.atlantis_atlantis.is_none());

        assert!(builder.atlantis_olympus.is_none());

        assert!(builder.olympus_atlantis.is_none());

        assert!(builder.olympus_olympus.is_none());
    }

    #[test]
    fn test_builder_customized() {
        let builder = GameOptions::builder("customized");

        assert!(builder.choice_atlantis.is_none());

        assert!(builder.choice_olympus.is_none());

        assert!(builder.atlantis_atlantis.is_none());

        assert!(builder.atlantis_olympus.is_none());

        assert!(builder.olympus_atlantis.is_none());

        assert!(builder.olympus_olympus.is_none());
    }
}
