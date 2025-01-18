// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{
    BuilderError,
    GameOptions,
    NumberPair,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameOptionsBuilderTypes {
    /// This builder variant allows for full randomization
    /// of the `GameOptions` struct that is being built.
    Randomized,
    /// This builder variant allows for the `GameOptions` struct
    /// that is being built to be randomized, but seeded.
    Seeded,
    /// This builder variant allows for the `GameOptions` struct
    /// that is being built to be fully customized.
    Customized,
}

/// A builder struct to create a [`GameOptions`](crate::GameOptions).
///
/// This struct is designed to encapsulate different ways of generating a new
/// `GameOptions` struct. It is designed to be used in conjunction with the
/// [`GameOptions`](crate::GameOptions) struct.
///
/// The `GameOptionsBuilder::new()` method is used to create a new builder,
/// which takes a `builder_type` argument. This argument is used to determine
/// which variant of the builder is used. The `builder_type` argument can be one
/// of the following:
///
/// * "randomized": This creates a builder of type
///   [`GameOptionsBuilderTypes::Randomized`](GameOptionsBuilderTypes::Randomized).
/// * "seeded": This creates a builder of type
///   [`GameOptionsBuilderTypes::Seeded`](GameOptionsBuilderTypes::Seeded)
/// * "customized": This creates a builder of type
///   [`GameOptionsBuilderTypes::Customized`](GameOptionsBuilderTypes::Customized)
///
/// Each of these variants has different fields that can be set and serve
/// different purposes. The following table shows which fields should be set for
/// each variant:
///
/// | Field | Randomized | Seeded | Customized | Type | Default |
/// | ----- | ---------- | ------ | ---------- | ---- | ------- |
/// | `min_value` | Yes | No | Yes | u32 | 1 |
/// | `max_value` | Yes | No | Yes | u32 | 10 |
/// | `choice_atlantis` | Yes | Yes | Yes | &'static str | "cooperate" |
/// | `choice_olympus` | Yes | Yes | Yes | &'static str | "defect" |
/// | `atlantis_atlantis` | No | No | Yes | `NumberPair` | `NumberPair::new(4, 4)` |
/// | `atlantis_olympus` | No | No | Yes | `NumberPair` | `NumberPair::new(0, 5)` |
/// | `olympus_atlantis` | No | No | Yes | `NumberPair` | `NumberPair::new(5, 0)` |
/// | `olympus_olympus` | No | No | Yes | `NumberPair` | `NumberPair::new(3, 3)` |
/// | `seed` | No | Yes | No | `u64` | `None` |
/// # Example
///
/// ## `RandomizedBuilder`
///
/// This builder path takes in the lower bound and the upper bound for the
/// scores that can be assigned to each option. It then generates a random
/// score for each option within the given bounds.
///
/// ```
/// use dilemma_tactix_lib::{
///     GameOptionsBuilder as Builder,
///     GameOptionsBuilderTypes as BuilderTypes,
///     NumberPair,
/// # BuilderError,
/// };
///
/// let builder = Builder::new(BuilderTypes::Randomized);
/// let builder = builder.min_value(1);
/// # assert!(builder.is_ok());
/// let builder = builder?.max_value(10);
/// # assert!(builder.is_ok());
/// let builder = builder?.choice_atlantis("cooperate");
/// # assert!(builder.is_ok());
/// let builder = builder?.choice_olympus("defect");
/// # assert!(builder.is_ok());
///
/// let game_options = builder?.build();
///
/// # assert_eq!(game_options.choice_atlantis(), "cooperate");
/// # assert_eq!(game_options.choice_olympus(), "defect");
/// # assert!(game_options.atlantis_atlantis().first() >= 1);
/// # assert!(game_options.atlantis_atlantis().first() <= 10);
/// # assert!(game_options.atlantis_atlantis().second() >= 1);
/// # assert!(game_options.atlantis_atlantis().second() <= 10);
/// # assert!(game_options.atlantis_olympus().first() >= 1);
/// # assert!(game_options.atlantis_olympus().first() <= 10);
/// # assert!(game_options.atlantis_olympus().second() >= 1);
/// # assert!(game_options.atlantis_olympus().second() <= 10);
/// # assert!(game_options.olympus_atlantis().first() >= 1);
/// # assert!(game_options.olympus_atlantis().first() <= 10);
/// # assert!(game_options.olympus_atlantis().second() >= 1);
/// # assert!(game_options.olympus_atlantis().second() <= 10);
/// # assert!(game_options.olympus_olympus().first() >= 1);
/// # assert!(game_options.olympus_olympus().first() <= 10);
/// # assert!(game_options.olympus_olympus().second() >= 1);
/// # assert!(game_options.olympus_olympus().second() <= 10);
///
/// # Ok::<(), BuilderError>(())
/// ```
/// ## `SeededBuilder`
///
/// This builder path takes in the lower bound and the upper bound for the
/// scores that can be assigned to each option. It also takes a `seed` value
/// It then generates a random score for each option within the given
/// bounds and the given seed.
///
/// ```no_run
/// use dilemma_tactix_lib::{
///     GameOptionsBuilder as Builder,
///     GameOptionsBuilderTypes as BuilderTypes,
/// };
///
/// let builder = Builder::new(BuilderTypes::Seeded);
/// # assert_eq!(builder.choice_atlantis, None);
/// # assert_eq!(builder.choice_olympus, None);
/// # assert_eq!(builder.atlantis_atlantis, None);
/// # assert_eq!(builder.atlantis_olympus, None);
/// # assert_eq!(builder.olympus_atlantis, None);
/// # assert_eq!(builder.olympus_olympus, None);
/// ```
///
/// ## `CustomizedBuilder`
///
/// This builder allows you to set all the scores for each possible
/// outcome individually. This allows for the most customization of the
/// `GameOptions` struct.
///
/// ```
/// use dilemma_tactix_lib::{
///     GameOptionsBuilder as Builder,
///     GameOptionsBuilderTypes as BuilderTypes,
///     NumberPair,
/// # BuilderError,
/// };
///
/// let builder = Builder::new(BuilderTypes::Customized);
/// let builder = builder.choice_atlantis("cooperate");
/// # assert!(builder.is_ok());
/// let builder = builder?.choice_olympus("defect");
/// # assert!(builder.is_ok());
/// let builder = builder?.atlantis_atlantis(NumberPair::new(4, 4));
/// # assert!(builder.is_ok());
/// let builder = builder?.atlantis_olympus(NumberPair::new(0, 5));
/// # assert!(builder.is_ok());
/// let builder = builder?.olympus_atlantis(NumberPair::new(5, 0));
/// # assert!(builder.is_ok());
/// let builder = builder?.olympus_olympus(NumberPair::new(3, 3));
/// # assert!(builder.is_ok());
///
/// let game_options = builder?.build();
///
/// # assert_eq!(game_options.choice_atlantis(), "cooperate");
/// # assert_eq!(game_options.choice_olympus(), "defect");
/// # assert_eq!(game_options.atlantis_atlantis(), NumberPair::new(4, 4));
/// # assert_eq!(game_options.atlantis_olympus(), NumberPair::new(0, 5));
/// # assert_eq!(game_options.olympus_atlantis(), NumberPair::new(5, 0));
/// # assert_eq!(game_options.olympus_olympus(), NumberPair::new(3, 3));
/// # Ok::<(), BuilderError>(())
/// ```
///
/// # Notes
///
/// I chose to use the `Builder` pattern here because it allows for easier
/// manipulation of the `GameOptions` struct. I also used a `State` or `Variant`
/// pattern here because it had the lowest complexity in terms of understanding
/// the flow of options that need to be set.
///
/// # See Also
///
/// * [`GameOptions`](crate::GameOptions)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameOptionsBuilder {
    builder_type:          GameOptionsBuilderTypes,
    min_value:             Option<u32>,
    max_value:             Option<u32>,
    pub choice_atlantis:   Option<&'static str>,
    pub choice_olympus:    Option<&'static str>,
    pub atlantis_atlantis: Option<NumberPair>,
    pub atlantis_olympus:  Option<NumberPair>,
    pub olympus_atlantis:  Option<NumberPair>,
    pub olympus_olympus:   Option<NumberPair>,
    seed:                  Option<u64>,
}

impl GameOptionsBuilder {
    /// Creates a new `GameOptionsBuilder` struct.
    ///
    /// This associated function creates a new `GameOptionsBuilder` struct with
    /// the given `builder_type`. The `builder_type` argument can be one of the
    /// following:
    ///
    /// * "randomized": This creates a builder of type
    ///   [`GameOptionsBuilderTypes::Randomized`](GameOptionsBuilderTypes::Randomized).
    /// * "seeded": This creates a builder of type
    ///   [`GameOptionsBuilderTypes::Seeded`](GameOptionsBuilderTypes::Seeded)
    /// * "customized": This creates a builder of type
    ///   [`GameOptionsBuilderTypes::Customized`](GameOptionsBuilderTypes::Customized)
    ///
    /// # Arguments
    ///
    /// * `builder_type` - The type of builder to create.
    ///
    /// # Example
    ///
    /// ## `RandomizedBuilder`
    ///
    /// This builder path takes in the lower bound and the upper bound for the
    /// scores that can be assigned to each option. It then generates a random
    /// score for each option within the given bounds.
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized);
    /// # assert_eq!(game_options_builder.choice_atlantis, None);
    /// # assert_eq!(game_options_builder.choice_olympus, None);
    /// # assert_eq!(game_options_builder.atlantis_atlantis, None);
    /// # assert_eq!(game_options_builder.atlantis_olympus, None);
    /// # assert_eq!(game_options_builder.olympus_atlantis, None);
    /// # assert_eq!(game_options_builder.olympus_olympus, None);
    /// ```
    /// ## `SeededBuilder`
    ///
    /// This builder path takes in the lower bound and the upper bound for the
    /// scores that can be assigned to each option. It also takes a `seed` value
    /// It then generates a random score for each option within the given
    /// bounds and the given seed.
    ///
    /// ```no_run
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded);
    /// # assert_eq!(game_options_builder.choice_atlantis, None);
    /// # assert_eq!(game_options_builder.choice_olympus, None);
    /// # assert_eq!(game_options_builder.atlantis_atlantis, None);
    /// # assert_eq!(game_options_builder.atlantis_olympus, None);
    /// # assert_eq!(game_options_builder.olympus_atlantis, None);
    /// # assert_eq!(game_options_builder.olympus_olympus, None);
    /// ```
    ///
    /// ## `CustomizedBuilder`
    ///
    /// This builder allows you to set all the scores for each possible
    /// outcome individually. This allows for the most customization of the
    /// `GameOptions` struct.
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized);
    /// # assert_eq!(game_options_builder.choice_atlantis, None);
    /// # assert_eq!(game_options_builder.choice_olympus, None);
    /// # assert_eq!(game_options_builder.atlantis_atlantis, None);
    /// # assert_eq!(game_options_builder.atlantis_olympus, None);
    /// # assert_eq!(game_options_builder.olympus_atlantis, None);
    /// # assert_eq!(game_options_builder.olympus_olympus, None);
    /// ```
    ///
    /// # Returns
    ///
    /// A new `GameOptionsBuilder` struct with a given builder type.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    /// * [`GameOptionsBuilder::seed()`](GameOptionsBuilder::seed())
    #[must_use]
    pub const fn new(builder_type: GameOptionsBuilderTypes) -> Self {
        Self {
            builder_type,
            max_value: None,
            min_value: None,
            choice_atlantis: None,
            choice_olympus: None,
            atlantis_atlantis: None,
            atlantis_olympus: None,
            olympus_atlantis: None,
            olympus_olympus: None,
            seed: None,
        }
    }

    /// Sets the minimum value for the `GameOptions`.
    ///
    /// This function sets the minimum value for the `GameOptions` struct that
    /// is being built. This function is only valid for the
    /// [`GameOptionsBuilderTypes::Randomized`](GameOptionsBuilderTypes::Randomized)
    /// and the [`GameOptionsBuilderTypes::Seeded`](GameOptionsBuilderTypes::Seeded)
    /// variants of the `GameOptionsBuilder` struct.
    ///
    /// # Arguments
    ///
    /// * `min_value` - The minimum value for the `GameOptions`.
    ///
    /// # Example
    ///
    /// ## `RandomizedBuilder`
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized)
    ///         .min_value(1);
    /// assert!(game_options_builder.is_ok());
    /// ```
    ///
    /// ## `SeededBuilder`
    ///
    /// ```no_run
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded).min_value(1);
    ///
    /// assert!(game_options_builder.is_ok());
    /// ```
    ///
    /// ## `CustomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized)
    ///         .min_value(1);
    ///
    /// assert!(game_options_builder.is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the `builder_type` is
    /// [`GameOptionsBuilderTypes::Customized`](GameOptionsBuilderTypes::Customized).
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `min_value` field set.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    /// * [`GameOptionsBuilder::seed()`](GameOptionsBuilder::seed())
    pub fn min_value(mut self, min_value: u32) -> Result<Self, BuilderError> {
        match self.builder_type {
            GameOptionsBuilderTypes::Randomized | GameOptionsBuilderTypes::Seeded => {
                self.min_value = Some(min_value);
                Ok(self)
            }
            GameOptionsBuilderTypes::Customized => Err(BuilderError::InvalidOptionSpecified(
                "min_value must not be set when using CusotmizedBuilder".to_string(),
            )),
        }
    }

    /// Sets the maximum value for the `GameOptions`.
    ///
    /// This function sets the maximum value for the `GameOptions` struct that
    /// is being built. This function is only valid for the
    /// [`GameOptionsBuilderTypes::Randomized`](GameOptionsBuilderTypes::Randomized)
    /// and the [`GameOptionsBuilderTypes::Seeded`](GameOptionsBuilderTypes::Seeded)
    /// variants of the `GameOptionsBuilder` struct.
    ///
    /// # Arguments
    ///
    /// * `max_value` - The maximum value for the `GameOptions`.
    ///
    /// # Example
    ///
    /// ## `RandomizedBuilder`
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized)
    ///         .max_value(10);
    /// assert!(game_options_builder.is_ok());
    /// ```
    ///
    /// ## `SeededBuilder`
    ///
    /// ```no_run
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded).max_value(10);
    ///
    /// assert!(game_options_builder.is_ok());
    /// ```
    ///
    /// ## `CustomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized)
    ///         .max_value(10);
    ///
    /// assert!(game_options_builder.is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the `builder_type` is
    /// [`GameOptionsBuilderTypes::Customized`](GameOptionsBuilderTypes::Customized).
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `max_value` field set.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    /// * [`GameOptionsBuilder::seed()`](GameOptionsBuilder::seed())
    pub fn max_value(mut self, max_value: u32) -> Result<Self, BuilderError> {
        match self.builder_type {
            GameOptionsBuilderTypes::Randomized | GameOptionsBuilderTypes::Seeded => {
                self.max_value = Some(max_value);
                Ok(self)
            }
            GameOptionsBuilderTypes::Customized => Err(BuilderError::InvalidOptionSpecified(
                "max_value must not be set when using CustomizedBuilder".to_string(),
            )),
        }
    }

    /// Sets the first choice available to players in `GameOptions`.
    ///
    /// This function sets the first choice available to players in
    /// `GameOptions`. This function is valid for all variants of the
    /// `GameOptionsBuilder` struct.
    ///
    /// # Arguments
    ///
    /// * `choice_atlantis` - The first choice available to players in `GameOptions`.
    ///
    /// # Example
    ///
    /// ## `RandomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized)
    ///         .choice_atlantis("cooperate");
    /// # assert!(game_options_builder.is_ok());
    ///
    /// let game_options = game_options_builder.unwrap().build();
    ///
    /// # assert_eq!(game_options.choice_atlantis(), "cooperate");
    /// ```
    ///
    /// ## `SeededBuilder`
    ///
    /// ```no_run
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    /// use rand::SeedableRng;
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded)
    ///         .choice_atlantis("cooperate");
    /// # assert!(game_options_builder.is_ok());
    ///
    /// let game_options = game_options_builder.unwrap().build();
    ///
    /// # assert_eq!(game_options.choice_atlantis(), "cooperate");
    /// ```
    ///
    /// ## `CustomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized)
    ///         .choice_atlantis("cooperate");
    /// # assert!(game_options_builder.is_ok());
    ///
    /// let game_options = game_options_builder.unwrap().build();
    ///
    /// # assert_eq!(game_options.choice_atlantis(), "cooperate");
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the `choice_atlantis` argument is
    /// empty.
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `choice_atlantis` field set.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    /// * [`GameOptionsBuilder::seed()`](GameOptionsBuilder::seed())
    pub fn choice_atlantis(mut self, choice_atlantis: &'static str) -> Result<Self, BuilderError> {
        if choice_atlantis.is_empty() {
            return Err(BuilderError::InvalidOptionValueSpecified(
                "choice_atlantis must not be empty".to_string(),
            ));
        }
        self.choice_atlantis = Some(choice_atlantis);
        Ok(self)
    }

    /// Sets the second choice available to players in `GameOptions`.
    ///
    /// This function sets the second choice available to players in
    /// `GameOptions`. This function is valid for all variants of the
    /// `GameOptionsBuilder` struct.
    ///
    /// # Arguments
    ///
    /// * `choice_olympus` - The second choice available to players in `GameOptions`.
    ///
    /// # Example
    ///
    /// ## `RandomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized)
    ///         .choice_olympus("defect");
    /// # assert!(game_options_builder.is_ok());
    ///
    /// let game_options = game_options_builder.unwrap().build();
    ///
    /// # assert_eq!(game_options.choice_olympus(), "defect");
    /// ```
    ///
    /// ## `SeededBuilder`
    ///
    /// ```no_run
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded)
    ///         .choice_olympus("defect");
    /// # assert!(game_options_builder.is_ok());
    ///
    /// let game_options = game_options_builder.unwrap().build();
    ///
    /// # assert_eq!(game_options.choice_olympus(), "defect");
    /// ```
    ///
    /// ## `CustomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized)
    ///         .choice_olympus("defect");
    /// # assert!(game_options_builder.is_ok());
    ///
    /// let game_options = game_options_builder.unwrap().build();
    ///
    /// # assert_eq!(game_options.choice_olympus(), "defect");
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the `choice_olympus` argument is
    /// empty.
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `choice_olympus` field set.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    /// * [`GameOptionsBuilder::seed()`](GameOptionsBuilder::seed())
    pub fn choice_olympus(mut self, choice_olympus: &'static str) -> Result<Self, BuilderError> {
        if choice_olympus.is_empty() {
            return Err(BuilderError::InvalidOptionValueSpecified(
                "choice_olympus must not be empty".to_string(),
            ));
        }
        self.choice_olympus = Some(choice_olympus);
        Ok(self)
    }

    /// Sets the score for the case when both players (Aleph and Beth) choose
    /// the first (Atlantis) choice.
    ///
    /// This function sets the score for the case when both players choose the
    /// first (Atlantis) choice. This function is only valid for the
    /// [`GameOptionsBuilderTypes::Customized`](GameOptionsBuilderTypes::Customized)
    /// variant of the `GameOptionsBuilder` struct.
    ///
    /// # Arguments
    ///
    /// * `atlantis_atlantis` - The score to set.
    ///
    /// # Example
    ///
    /// ## `RandomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized)
    ///         .atlantis_atlantis(NumberPair::new(4, 4));
    /// # assert!(game_options_builder.is_err());
    /// ```
    ///
    /// ## `SeededBuilder`
    ///
    /// ```no_run
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded)
    ///         .atlantis_atlantis(NumberPair::new(4, 4));
    /// # assert!(game_options_builder.is_err());
    /// ```
    ///
    /// ## `CustomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///    GameOptionsBuilder,
    ///   GameOptionsBuilderTypes,
    ///    NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///    GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized)
    ///       .atlantis_atlantis(NumberPair::new(4, 4));
    ///
    /// # assert!(game_options_builder.is_ok());
    ///
    /// let game_options = game_options_builder.unwrap().build();
    ///
    /// # assert_eq!(game_options.atlantis_atlantis(), NumberPair::new(4, 4));
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the `builder_type` is
    /// [`GameOptionsBuilderTypes::Randomized`](GameOptionsBuilderTypes::Randomized)
    /// or [`GameOptionsBuilderTypes::Seeded`](GameOptionsBuilderTypes::Seeded).
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `atlantis_atlantis` field set.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    /// * [`GameOptionsBuilder::seed()`](GameOptionsBuilder::seed())
    pub fn atlantis_atlantis(
        mut self,
        atlantis_atlantis: NumberPair,
    ) -> Result<Self, BuilderError> {
        match self.builder_type {
            GameOptionsBuilderTypes::Randomized => Err(BuilderError::InvalidOptionSpecified(
                "Field atlantis_atlantis can not be set when using RandomizedBuilder".to_string(),
            )),
            GameOptionsBuilderTypes::Seeded => Err(BuilderError::InvalidOptionSpecified(
                "Field atlantis_atlantis can not be set when using SeededBuilder".to_string(),
            )),
            GameOptionsBuilderTypes::Customized => {
                self.atlantis_atlantis = Some(atlantis_atlantis);
                Ok(self)
            }
        }
    }

    /// Sets the score for the case when Player Aleph chooses the first
    /// (Atlantis) choice and Player Beth chooses the second (Olympus)
    /// choice.
    ///
    /// This function sets the score for the case when Player Aleph chooses the
    /// first (Atlantis) choice and Player Beth chooses the second (Olympus)
    /// choice. This function is only valid for the
    /// [`GameOptionsBuilderTypes::Customized`](GameOptionsBuilderTypes::Customized)
    /// variant of the `GameOptionsBuilder` struct.
    ///
    /// # Arguments
    ///
    /// * `atlantis_olympus` - The score to set.
    ///
    /// # Example
    ///
    /// ## `RandomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized)
    ///         .atlantis_olympus(NumberPair::new(0, 5));
    /// # assert!(game_options_builder.is_err());
    /// ```
    ///
    /// ## `SeededBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded)
    ///         .atlantis_olympus(NumberPair::new(0, 5));
    /// # assert!(game_options_builder.is_err());
    /// ```
    ///
    /// ## `CustomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized)
    ///         .atlantis_olympus(NumberPair::new(0, 5));
    /// # assert!(game_options_builder.is_ok());
    ///
    /// let game_options = game_options_builder.unwrap().build();
    ///
    /// # assert_eq!(game_options.atlantis_olympus(), NumberPair::new(0, 5));
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the `builder_type` is
    /// [`GameOptionsBuilderTypes::Randomized`](GameOptionsBuilderTypes::Randomized)
    /// or [`GameOptionsBuilderTypes::Seeded`](GameOptionsBuilderTypes::Seeded).
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `atlantis_olympus` field set.
    ///
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    /// * [`GameOptionsBuilder::seed()`](GameOptionsBuilder::seed())
    pub fn atlantis_olympus(mut self, atlantis_olympus: NumberPair) -> Result<Self, BuilderError> {
        match self.builder_type {
            GameOptionsBuilderTypes::Randomized => Err(BuilderError::InvalidOptionSpecified(
                "Field atlantis_olympus can not be set when using RandomizedBuilder".to_string(),
            )),
            GameOptionsBuilderTypes::Seeded => Err(BuilderError::InvalidOptionSpecified(
                "Field atlantis_olympus can not be set when using SeededBuilder".to_string(),
            )),
            GameOptionsBuilderTypes::Customized => {
                self.atlantis_olympus = Some(atlantis_olympus);
                Ok(self)
            }
        }
    }

    /// Sets the score for the case when Player Aleph chooses the second
    /// choice and Player Beth chooses the first choice.
    ///
    /// This function sets the score for the case when Player Aleph chooses the
    /// second (Olympus) choice and Player Beth chooses the first (Atlantis)
    /// choice. This function is only valid for the
    /// [`GameOptionsBuilderTypes::Customized`](GameOptionsBuilderTypes::Customized)
    /// variant of the `GameOptionsBuilder` struct.
    ///
    /// # Arguments
    ///
    /// * `olympus_atlantis` - The score to set.
    ///
    /// # Example
    ///
    /// ## `RandomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized)
    ///         .olympus_atlantis(NumberPair::new(5, 0));
    ///
    /// # assert!(game_options_builder.is_err());
    /// ```
    ///
    /// ## `SeededBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded)
    ///         .olympus_atlantis(NumberPair::new(5, 0));
    ///
    /// # assert!(game_options_builder.is_err());
    /// ```
    ///
    /// ## `CustomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized)
    ///         .olympus_atlantis(NumberPair::new(5, 0));
    ///
    /// # assert!(game_options_builder.is_ok());
    ///
    /// let game_options = game_options_builder.unwrap().build();
    ///
    /// # assert_eq!(game_options.olympus_atlantis(), NumberPair::new(5, 0));
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the `builder_type` is
    /// [`GameOptionsBuilderTypes::Randomized`](GameOptionsBuilderTypes::Randomized)
    /// or [`GameOptionsBuilderTypes::Seeded`](GameOptionsBuilderTypes::Seeded).
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `olympus_atlantis` field set.
    ///
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    /// * [`GameOptionsBuilder::seed()`](GameOptionsBuilder::seed())
    pub fn olympus_atlantis(mut self, olympus_atlantis: NumberPair) -> Result<Self, BuilderError> {
        match self.builder_type {
            GameOptionsBuilderTypes::Randomized => Err(BuilderError::InvalidOptionSpecified(
                "Field olympus_atlantis can not be set when using RandomizedBuilder".to_string(),
            )),
            GameOptionsBuilderTypes::Seeded => Err(BuilderError::InvalidOptionSpecified(
                "Field olympus_atlantis can not be set when using SeededBuilder".to_string(),
            )),
            GameOptionsBuilderTypes::Customized => {
                self.olympus_atlantis = Some(olympus_atlantis);
                Ok(self)
            }
        }
    }

    /// Sets the score for the case when both players (Aleph and Beth) choose
    /// the second (Olympus) choice.
    ///
    /// This function sets the score for the case when both players choose the
    /// second (Olympus) choice. This function is only valid for the
    /// [`GameOptionsBuilderTypes::Customized`](GameOptionsBuilderTypes::Customized)
    /// variant of the `GameOptionsBuilder` struct.
    ///
    /// # Arguments
    ///
    /// * `olympus_olympus` - The score to set.
    ///
    /// # Example
    ///
    /// ## `RandomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized)
    ///         .olympus_olympus(NumberPair::new(3, 3));
    ///
    /// # assert!(game_options_builder.is_err());
    /// ```
    ///
    /// ## `SeededBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded)
    ///         .olympus_olympus(NumberPair::new(3, 3));
    ///
    /// # assert!(game_options_builder.is_err());
    /// ```
    ///
    /// ## `CustomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized)
    ///         .olympus_olympus(NumberPair::new(3, 3));
    ///
    /// # assert!(game_options_builder.is_ok());
    ///
    /// let game_options = game_options_builder.unwrap().build();
    ///
    /// # assert_eq!(game_options.olympus_olympus(), NumberPair::new(3, 3));
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the `builder_type` is
    /// [`GameOptionsBuilderTypes::Randomized`](GameOptionsBuilderTypes::Randomized)
    /// or [`GameOptionsBuilderTypes::Seeded`](GameOptionsBuilderTypes::Seeded).
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `olympus_olympus` field set.
    ///
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::seed()`](GameOptionsBuilder::seed())
    pub fn olympus_olympus(mut self, olympus_olympus: NumberPair) -> Result<Self, BuilderError> {
        match self.builder_type {
            GameOptionsBuilderTypes::Randomized => Err(BuilderError::InvalidOptionSpecified(
                "Field olympus_olympus can not be set when using RandomizedBuilder".to_string(),
            )),
            GameOptionsBuilderTypes::Seeded => Err(BuilderError::InvalidOptionSpecified(
                "Field olympus_olympus can not be set when using SeededBuilder".to_string(),
            )),
            GameOptionsBuilderTypes::Customized => {
                self.olympus_olympus = Some(olympus_olympus);
                Ok(self)
            }
        }
    }

    /// Sets the seed for the `GameOptions`.
    ///
    /// This function sets the seed for the `GameOptions` struct that is being
    /// built. This function is only valid for the
    /// [`GameOptionsBuilderTypes::Seeded`](GameOptionsBuilderTypes::Seeded)
    /// variant of the `GameOptionsBuilder` struct.
    ///
    /// # Arguments
    ///
    /// * `seed` - The seed for the `GameOptions`.
    ///
    /// # Example
    ///
    /// ## `RandomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized)
    ///         .seed(123456789);
    ///
    /// # assert!(game_options_builder.is_err());
    /// ```
    ///
    /// ## `SeededBuilder`
    ///
    /// ```no_run
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded)
    ///         .seed(123456789);
    ///
    /// # assert!(game_options_builder.is_ok());
    /// ```
    ///
    /// ## `CustomizedBuilder`
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     GameOptionsBuilderTypes,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized)
    ///         .seed(123456789);
    ///
    /// # assert!(game_options_builder.is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the `builder_type` is
    /// [`GameOptionsBuilderTypes::Randomized`](GameOptionsBuilderTypes::Randomized)
    /// or [`GameOptionsBuilderTypes::Customized`](GameOptionsBuilderTypes::Customized).
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `seed` field set.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    pub fn seed(mut self, seed: u64) -> Result<Self, BuilderError> {
        match self.builder_type {
            GameOptionsBuilderTypes::Randomized => Err(BuilderError::InvalidOptionSpecified(
                "Field seed can not be set when using RandomizedBuilder".to_string(),
            )),
            GameOptionsBuilderTypes::Seeded => {
                self.seed = Some(seed);
                Ok(self)
            }
            GameOptionsBuilderTypes::Customized => Err(BuilderError::InvalidOptionSpecified(
                "Field seed can not be set when using CustomizedBuilder".to_string(),
            )),
        }
    }

    /// Builds the `GameOptions` struct.
    ///
    /// # Returns
    ///
    /// A new `GameOptions` struct.
    ///
    /// # Panics
    ///
    /// This function will panic if any of the required fields are not set.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    #[must_use]
    pub fn build(self) -> GameOptions {
        match self.builder_type {
            GameOptionsBuilderTypes::Randomized => self.build_randomized(),
            GameOptionsBuilderTypes::Seeded => self.build_seeded(),
            GameOptionsBuilderTypes::Customized => self.build_customized(),
        }
    }

    fn build_customized(&self) -> GameOptions {
        let choice_atlantis = self.choice_atlantis.unwrap_or("cooperate");
        let choice_olympus = self.choice_olympus.unwrap_or("defect");
        let atlantis_atlantis = self.atlantis_atlantis.unwrap_or(NumberPair::new(4, 4));
        let atlantis_olympus = self.atlantis_olympus.unwrap_or(NumberPair::new(5, 0));
        let olympus_atlantis = self.olympus_atlantis.unwrap_or(NumberPair::new(0, 5));
        let olympus_olympus = self.olympus_olympus.unwrap_or(NumberPair::new(3, 3));

        GameOptions {
            choice_atlantis,
            choice_olympus,
            atlantis_atlantis,
            atlantis_olympus,
            olympus_atlantis,
            olympus_olympus,
        }
    }

    fn build_seeded(&self) -> GameOptions {
        todo!()
    }

    fn build_randomized(&self) -> GameOptions {
        let min_value = self.min_value.unwrap_or(1);
        let max_value = self.max_value.unwrap_or(10);

        let choice_atlantis = self.choice_atlantis.unwrap_or("cooperate");
        let choice_olympus = self.choice_olympus.unwrap_or("defect");

        let atlantis_atlantis = NumberPair::random(min_value, max_value);
        let atlantis_olympus = NumberPair::random(min_value, max_value);
        let olympus_atlantis = NumberPair::random(min_value, max_value);
        let olympus_olympus = NumberPair::random(min_value, max_value);

        GameOptions {
            choice_atlantis,
            choice_olympus,
            atlantis_atlantis,
            atlantis_olympus,
            olympus_atlantis,
            olympus_olympus,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let game_options_builder_randomized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized);
        assert_eq!(
            game_options_builder_randomized.builder_type,
            GameOptionsBuilderTypes::Randomized
        );
        let game_options_builder_seeded = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded);
        assert_eq!(
            game_options_builder_seeded.builder_type,
            GameOptionsBuilderTypes::Seeded
        );
        let game_options_builder_customized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized);
        assert_eq!(
            game_options_builder_customized.builder_type,
            GameOptionsBuilderTypes::Customized
        );
    }

    #[test]
    fn test_min_value() {
        let game_options_builder_randomized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized).min_value(1);
        assert!(game_options_builder_randomized.is_ok());
        let game_options_builder_seeded =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded).min_value(1);
        assert!(game_options_builder_seeded.is_ok());
        let game_options_builder_customized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized).min_value(1);
        assert!(game_options_builder_customized.is_err());
    }

    #[test]
    fn test_max_value() {
        let game_options_builder_randomized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized).max_value(10);
        assert!(game_options_builder_randomized.is_ok());
        let game_options_builder_seeded =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded).max_value(10);
        assert!(game_options_builder_seeded.is_ok());
        let game_options_builder_customized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized).max_value(10);
        assert!(game_options_builder_customized.is_err());
    }

    #[test]
    fn test_choice_atlantis() {
        let game_options_builder_randomized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized)
                .choice_atlantis("cooperate");
        assert!(game_options_builder_randomized.is_ok());
        let game_options_builder_seeded =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded).choice_atlantis("cooperate");
        assert!(game_options_builder_seeded.is_ok());
        let game_options_builder_customized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized)
                .choice_atlantis("cooperate");
        assert!(game_options_builder_customized.is_ok());
    }

    #[test]
    fn test_choice_olympus() {
        let game_options_builder_randomized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized).choice_olympus("defect");
        assert!(game_options_builder_randomized.is_ok());
        let game_options_builder_seeded =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded).choice_olympus("defect");
        assert!(game_options_builder_seeded.is_ok());
        let game_options_builder_customized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized).choice_olympus("defect");
        assert!(game_options_builder_customized.is_ok());
    }

    #[test]
    fn test_atlantis_atlantis() {
        let game_options_builder_randomized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized)
                .atlantis_atlantis(NumberPair::new(1, 1));
        assert!(game_options_builder_randomized.is_err());
        let game_options_builder_seeded = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded)
            .atlantis_atlantis(NumberPair::new(1, 1));
        assert!(game_options_builder_seeded.is_err());
        let game_options_builder_customized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized)
                .atlantis_atlantis(NumberPair::new(1, 1));
        assert!(game_options_builder_customized.is_ok());
    }

    #[test]
    fn test_atlantis_olympus() {
        let game_options_builder_randomized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized)
                .atlantis_olympus(NumberPair::new(1, 1));
        assert!(game_options_builder_randomized.is_err());
        let game_options_builder_seeded = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded)
            .atlantis_olympus(NumberPair::new(1, 1));
        assert!(game_options_builder_seeded.is_err());
        let game_options_builder_customized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized)
                .atlantis_olympus(NumberPair::new(1, 1));
        assert!(game_options_builder_customized.is_ok());
    }

    #[test]
    fn test_olympus_atlantis() {
        let game_options_builder_randomized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized)
                .olympus_atlantis(NumberPair::new(1, 1));
        assert!(game_options_builder_randomized.is_err());
        let game_options_builder_seeded = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded)
            .olympus_atlantis(NumberPair::new(1, 1));
        assert!(game_options_builder_seeded.is_err());
        let game_options_builder_customized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized)
                .olympus_atlantis(NumberPair::new(1, 1));
        assert!(game_options_builder_customized.is_ok());
    }

    #[test]
    fn test_olympus_olympus() {
        let game_options_builder_randomized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized)
                .olympus_olympus(NumberPair::new(1, 1));
        assert!(game_options_builder_randomized.is_err());
        let game_options_builder_seeded = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded)
            .olympus_olympus(NumberPair::new(1, 1));
        assert!(game_options_builder_seeded.is_err());
        let game_options_builder_customized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized)
                .olympus_olympus(NumberPair::new(1, 1));
        assert!(game_options_builder_customized.is_ok());
    }

    #[test]
    fn test_seed() {
        let game_options_builder_randomized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized).seed(123456789);
        assert!(game_options_builder_randomized.is_err());
        let game_options_builder_seeded =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded).seed(123456789);
        assert!(game_options_builder_seeded.is_ok());
        let game_options_builder_customized =
            GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized).seed(123456789);
        assert!(game_options_builder_customized.is_err());
    }

    #[test]
    fn test_build_randomized() -> Result<(), BuilderError> {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized);
        let builder = builder.min_value(1);
        assert!(builder.is_ok());
        let builder = builder?.max_value(10);
        assert!(builder.is_ok());
        let builder = builder?.choice_atlantis("cooperate");
        assert!(builder.is_ok());
        let builder = builder?.choice_olympus("defect");
        assert!(builder.is_ok());
        let game_options = builder?.build();

        assert_eq!(game_options.choice_atlantis(), "cooperate");
        assert_eq!(game_options.choice_olympus(), "defect");

        Ok(())
    }

    #[test]
    #[ignore = "Seeded is not yet implemented"]
    fn test_build_seeded() -> Result<(), BuilderError> {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded);
        let builder = builder.min_value(1);
        assert!(builder.is_ok());
        let builder = builder?.max_value(10);
        assert!(builder.is_ok());
        let builder = builder?.choice_atlantis("cooperate");
        assert!(builder.is_ok());
        let builder = builder?.choice_olympus("defect");
        assert!(builder.is_ok());
        let builder = builder?.seed(123456789);
        assert!(builder.is_ok());
        let game_options = builder?.build();

        assert_eq!(game_options.choice_atlantis(), "cooperate");
        assert_eq!(game_options.choice_olympus(), "defect");

        Ok(())
    }

    #[test]
    fn test_build_customized() -> Result<(), BuilderError> {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized);
        let builder = builder.choice_atlantis("cooperate");
        assert!(builder.is_ok());
        let builder = builder?.choice_olympus("defect");
        assert!(builder.is_ok());
        let builder = builder?.atlantis_atlantis(NumberPair::new(1, 1));
        assert!(builder.is_ok());
        let builder = builder?.atlantis_olympus(NumberPair::new(1, 1));
        assert!(builder.is_ok());
        let builder = builder?.olympus_atlantis(NumberPair::new(1, 1));
        assert!(builder.is_ok());
        let builder = builder?.olympus_olympus(NumberPair::new(1, 1));
        assert!(builder.is_ok());
        let game_options = builder?.build();

        assert_eq!(game_options.choice_atlantis(), "cooperate");
        assert_eq!(game_options.choice_olympus(), "defect");

        Ok(())
    }

    #[test]
    fn test_build_randomized_missing_min_value() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized);
        let builder = builder.max_value(10);
        assert!(builder.is_ok());
        let builder = builder.unwrap().choice_atlantis("cooperate");
        assert!(builder.is_ok());
        let builder = builder.unwrap().choice_olympus("defect");
        assert!(builder.is_ok());
        let game_options = builder.unwrap().build();

        assert_eq!(game_options.choice_atlantis(), "cooperate");
        assert_eq!(game_options.choice_olympus(), "defect");
    }

    #[test]
    fn test_build_randomized_missing_max_value() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized);
        let builder = builder.min_value(1);
        assert!(builder.is_ok());
        let builder = builder.unwrap().choice_atlantis("cooperate");
        assert!(builder.is_ok());
        let builder = builder.unwrap().choice_olympus("defect");
        assert!(builder.is_ok());
        let game_options = builder.unwrap().build();

        assert_eq!(game_options.choice_atlantis(), "cooperate");
        assert_eq!(game_options.choice_olympus(), "defect");
    }

    #[test]
    fn test_build_randomized_missing_choice_atlantis() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized);
        let builder = builder.min_value(1);
        assert!(builder.is_ok());
        let builder = builder.unwrap().max_value(10);
        assert!(builder.is_ok());
        let builder = builder.unwrap().choice_olympus("defect");
        assert!(builder.is_ok());
        let game_options = builder.unwrap().build();

        assert_eq!(game_options.choice_atlantis(), "cooperate");
        assert_eq!(game_options.choice_olympus(), "defect");
    }

    #[test]
    fn test_build_randomized_missing_choice_olympus() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized);
        let builder = builder.min_value(1);
        assert!(builder.is_ok());
        let builder = builder.unwrap().max_value(10);
        assert!(builder.is_ok());
        let builder = builder.unwrap().choice_atlantis("cooperate");
        assert!(builder.is_ok());
        let game_options = builder.unwrap().build();

        assert_eq!(game_options.choice_atlantis(), "cooperate");
        assert_eq!(game_options.choice_olympus(), "defect");
    }

    #[test]
    #[ignore = "Seeded build is not yet implemented"]
    fn test_build_seeded_missing_min_value() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded);
        let builder = builder.max_value(10);
        assert!(builder.is_ok());
        let builder = builder.unwrap().choice_atlantis("cooperate");
        assert!(builder.is_ok());
        let builder = builder.unwrap().choice_olympus("defect");
        assert!(builder.is_ok());
        let builder = builder.unwrap().seed(123456789);
        assert!(builder.is_ok());
        let game_options = builder.unwrap().build();

        assert_eq!(game_options.choice_atlantis(), "cooperate");
        assert_eq!(game_options.choice_olympus(), "defect");
    }

    #[test]
    #[ignore = "Seeded is not yet implemented"]
    fn test_build_seeded_missing_max_value() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded);
        let builder = builder.min_value(1);
        assert!(builder.is_ok());
        let builder = builder.unwrap().choice_atlantis("cooperate");
        assert!(builder.is_ok());
        let builder = builder.unwrap().choice_olympus("defect");
        assert!(builder.is_ok());
        let builder = builder.unwrap().seed(123456789);
        assert!(builder.is_ok());
        let game_options = builder.unwrap().build();

        assert_eq!(game_options.choice_atlantis(), "cooperate");
        assert_eq!(game_options.choice_olympus(), "defect");
    }

    #[test]
    #[ignore = "Seeded is not yet implemented"]
    fn test_build_seeded_missing_choice_atlantis() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded);
        let builder = builder.min_value(1);
        assert!(builder.is_ok());
        let builder = builder.unwrap().max_value(10);
        assert!(builder.is_ok());
        let builder = builder.unwrap().choice_olympus("defect");
        assert!(builder.is_ok());
        let builder = builder.unwrap().seed(123456789);
        assert!(builder.is_ok());
        let game_options = builder.unwrap().build();

        assert_eq!(game_options.choice_atlantis(), "cooperate");
        assert_eq!(game_options.choice_olympus(), "defect");
    }

    #[test]
    #[ignore = "Seeded is not yet implemented"]
    fn test_build_seeded_missing_choice_olympus() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded);
        let builder = builder.min_value(1);
        assert!(builder.is_ok());
        let builder = builder.unwrap().max_value(10);
        assert!(builder.is_ok());
        let builder = builder.unwrap().choice_atlantis("cooperate");
        assert!(builder.is_ok());
        let builder = builder.unwrap().seed(123456789);
        assert!(builder.is_ok());
        let game_options = builder.unwrap().build();

        assert_eq!(game_options.choice_atlantis(), "cooperate");
        assert_eq!(game_options.choice_olympus(), "defect");
    }

    #[test]
    #[ignore = "Seeded is not yet implemented"]
    fn test_build_seeded_missing_seed() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded);
        let builder = builder.min_value(1);
        assert!(builder.is_ok());
        let builder = builder.unwrap().max_value(10);
        assert!(builder.is_ok());
        let builder = builder.unwrap().choice_atlantis("cooperate");
        assert!(builder.is_ok());
        let builder = builder.unwrap().choice_olympus("defect");
        assert!(builder.is_ok());
        let game_options = builder.unwrap().build();

        assert_eq!(game_options.choice_atlantis(), "cooperate");
        assert_eq!(game_options.choice_olympus(), "defect");
    }

    #[test]
    #[should_panic]
    fn test_build_randomized_seed_panic() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized);
        let builder = builder.seed(123456789);
        assert!(builder.is_err());
        builder.unwrap();
    }

    #[test]
    #[should_panic]
    fn test_build_randomized_atlantis_atlantis_panic() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized);
        let builder = builder.atlantis_atlantis(NumberPair::new(1, 1));
        assert!(builder.is_err());
        builder.unwrap();
    }

    #[test]
    #[should_panic]
    fn test_build_randomized_atlantis_olympus_panic() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized);
        let builder = builder.atlantis_olympus(NumberPair::new(1, 1));
        assert!(builder.is_err());
        builder.unwrap();
    }

    #[test]
    #[should_panic]
    fn test_build_randomized_olympus_atlantis_panic() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized);
        let builder = builder.olympus_atlantis(NumberPair::new(1, 1));
        assert!(builder.is_err());
        builder.unwrap();
    }

    #[test]
    #[should_panic]
    fn test_build_randomized_olympus_olympus_panic() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Randomized);
        let builder = builder.olympus_olympus(NumberPair::new(1, 1));
        assert!(builder.is_err());
        builder.unwrap();
    }

    #[test]
    #[should_panic]
    fn test_build_seeded_atlantis_atlantis_panic() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded);
        let builder = builder.atlantis_atlantis(NumberPair::new(1, 1));
        assert!(builder.is_err());
        builder.unwrap();
    }

    #[test]
    #[should_panic]
    fn test_build_seeded_atlantis_olympus_panic() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded);
        let builder = builder.atlantis_olympus(NumberPair::new(1, 1));
        assert!(builder.is_err());
        builder.unwrap();
    }

    #[test]
    #[should_panic]
    fn test_build_seeded_olympus_atlantis_panic() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded);
        let builder = builder.olympus_atlantis(NumberPair::new(1, 1));
        assert!(builder.is_err());
        builder.unwrap();
    }

    #[test]
    #[should_panic]
    fn test_build_seeded_olympus_olympus_panic() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Seeded);
        let builder = builder.olympus_olympus(NumberPair::new(1, 1));
        assert!(builder.is_err());
        builder.unwrap();
    }

    #[test]
    #[should_panic]
    fn test_build_customized_seed_panic() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized);
        let builder = builder.seed(123456789);
        assert!(builder.is_err());
        builder.unwrap();
    }

    #[test]
    #[should_panic]
    fn test_build_customized_min_value_panic() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized);
        let builder = builder.min_value(1);
        assert!(builder.is_err());
        builder.unwrap();
    }

    #[test]
    #[should_panic]
    fn test_build_customized_max_value_panic() {
        let builder = GameOptionsBuilder::new(GameOptionsBuilderTypes::Customized);
        let builder = builder.max_value(10);
        assert!(builder.is_err());
        builder.unwrap();
    }
}
