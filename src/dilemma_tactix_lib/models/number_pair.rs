// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt::{
    self,
    Display,
    Formatter,
};

use rand::{
    Rng,
    SeedableRng,
};
use rand_chacha::ChaCha12Rng;

/// A convenience struct to represent a pair of numbers.
///
/// This struct is primarily here to be used as a stand-in for the
/// prize values for the Dilemma game.
///
/// The `default` constructor will create a `NumberPair` with random values
/// between 0 and 9 (inclusive) for `first` and between 1 and 10 (inclusive)
/// for `second`.
///
/// # Example
///
/// ## Using `NumberPair` with the `new` constructor
/// ```
/// use dilemma_tactix_lib::NumberPair;
///
/// let number_pair = NumberPair::new(1, 2);
///
/// assert_eq!(number_pair.first(), 1);
///
/// assert_eq!(number_pair.second(), 2);
/// ```
///
/// ## Using `NumberPair` with the `default` constructor
/// ```
/// use dilemma_tactix_lib::NumberPair;
///
/// let number_pair = NumberPair::default();
///
/// assert!(number_pair.first() >= 1);
///
/// assert!(number_pair.first() < 10);
///
/// assert!(number_pair.second() >= 1);
///
/// assert!(number_pair.second() < 10);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NumberPair {
    /// The first number in the pair.
    first:  u32,
    /// The second number in the pair.
    second: u32,
}

impl NumberPair {
    /// Creates a new `NumberPair` struct.
    ///
    /// # Arguments
    ///
    /// * `first` - The first number in the pair.
    /// * `second` - The second number in the pair.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::NumberPair;
    ///
    /// let number_pair = NumberPair::new(1, 2);
    ///
    /// assert_eq!(number_pair.first(), 1);
    ///
    /// assert_eq!(number_pair.second(), 2);
    /// ```
    ///
    /// # Returns
    ///
    /// A new `NumberPair` struct with the given values.
    #[must_use]
    pub const fn new(first: u32, second: u32) -> Self {
        Self { first, second }
    }

    /// Creates a new `NumberPair` struct with random values.
    ///
    /// # Arguments
    ///
    /// * `min_value` - The minimum value for that can be assigned to a choice.
    /// * `max_value` - The maximum value for that can be assigned to a choice.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::NumberPair;
    ///
    /// let number_pair = NumberPair::random(1, 10);
    ///
    /// assert!(number_pair.first() >= 1);
    ///
    /// assert!(number_pair.first() <= 10);
    ///
    /// assert!(number_pair.second() >= 1);
    ///
    /// assert!(number_pair.second() <= 10);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `min_value` is greater than `max_value`.
    ///
    /// # Returns
    ///
    /// A new `NumberPair` struct with random values between `min_value` and
    /// `max_value` for each of `first` and `second`.
    #[must_use]
    pub fn random(min_value: u32, max_value: u32) -> Self {
        let mut rng = ChaCha12Rng::from_entropy();

        if min_value < max_value {
            Self::new(
                rng.gen_range(min_value..=max_value),
                rng.gen_range(min_value..=max_value),
            )
        } else {
            panic!("min_value must be less than max_value");
        }
    }

    /// Creates a new `NumberPair` struct with random values with a given seed.
    ///
    /// This function is used for testing purposes only.
    ///
    /// # Arguments
    ///
    /// * `min_value` - The minimum value for that can be assigned to a choice.
    /// * `max_value` - The maximum value for that can be assigned to a choice.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     NumberPair,
    ///     RAND_SEED,
    /// };
    ///
    /// let number_pair = NumberPair::random_seeded(1, 10, RAND_SEED);
    ///
    /// assert!(number_pair.first() <= 10);
    ///
    /// assert!(number_pair.second() <= 10);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `min_value` is greater than `max_value`.
    ///
    /// # Returns
    ///
    /// A new `NumberPair` struct with random values between `min_value` and
    /// `max_value` for each of `first` and `second`.
    #[doc(hidden)]
    #[cfg(test)]
    pub(crate) fn random_seeded(min_value: u32, max_value: u32, seed: u64) -> Self {
        // Create a new RNG seeded with the given seed.
        let mut rng = ChaCha12Rng::seed_from_u64(seed);

        if min_value < max_value {
            Self::new(
                rng.gen_range(min_value..=max_value),
                rng.gen_range(min_value..=max_value),
            )
        } else {
            panic!("min_value must be less than max_value");
        }
    }

    /// Returns the value of `first`.
    ///
    /// # Example
    ///
    /// ## Using `NumberPair` with the `new` constructor
    /// ```
    /// use dilemma_tactix_lib::NumberPair;
    ///
    /// let number_pair = NumberPair::new(1, 2);
    ///
    /// assert_eq!(number_pair.first(), 1);
    /// ```
    ///
    /// ## Using `NumberPair` with the `default` constructor
    ///
    /// ```
    /// use dilemma_tactix_lib::NumberPair;
    ///
    /// let number_pair = NumberPair::default();
    ///
    /// assert!(number_pair.first() >= 1);
    ///
    /// assert!(number_pair.first() < 10);
    /// ```
    ///
    /// # Returns
    ///
    /// The value of `first` as a `u32`.
    #[must_use]
    pub const fn first(&self) -> u32 {
        self.first
    }

    /// Returns the value of `second`.
    ///
    /// # Example
    ///
    /// ## Using `NumberPair` with the `new` constructor
    ///
    /// ```
    /// use dilemma_tactix_lib::NumberPair;
    ///
    /// let number_pair = NumberPair::new(1, 2);
    ///
    /// assert_eq!(number_pair.second(), 2);
    /// ```
    ///
    /// ## Using `NumberPair` with the `default` constructor
    /// ```
    /// use dilemma_tactix_lib::NumberPair;
    ///
    /// let number_pair = NumberPair::default();
    ///
    /// assert!(number_pair.second() >= 1);
    ///
    /// assert!(number_pair.second() < 10);
    /// ```
    ///
    /// # Returns
    ///
    /// The value of `second` as a `u32`.
    #[must_use]
    pub const fn second(&self) -> u32 {
        self.second
    }
}

/// Implements the `Display` trait for `NumberPair`.
///
/// This allows a `NumberPair` to be formatted as a string using the `{}` format
/// specifier. The resulting string will be in the format "(first, second)".
///
/// # Arguments
///
/// * `f` - A mutable reference to a `Formatter`. This is a context in which the formatting will be
///   done.
///
/// # Returns
///
/// This function returns a `fmt::Result`. If the write operation was
/// successful, the function returns `Ok(())`. If the write operation failed,
/// the function returns `Err` with the underlying error.
impl Display for NumberPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.first, self.second)
    }
}

impl Default for NumberPair {
    /// Creates a new `NumberPair` struct with default values.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::NumberPair;
    ///
    /// let number_pair = NumberPair::default();
    ///
    /// assert!(number_pair.first() >= 1);
    ///
    /// assert!(number_pair.first() < 10);
    ///
    /// assert!(number_pair.second() >= 1);
    ///
    /// assert!(number_pair.second() < 10);
    /// ```
    ///
    /// # Returns
    ///
    /// A new `NumberPair` struct with random values between 1 and 10 for each
    /// of `first` and `second`.
    fn default() -> Self {
        let mut rng = ChaCha12Rng::from_entropy();

        Self::new(rng.gen_range(1..10), rng.gen_range(1..10))
    }
}

#[cfg(test)]
#[allow(unused_comparisons)]
mod tests {

    use rstest::{
        fixture,
        rstest,
    };

    use super::*;
    use crate::constants::RANDOM_SEED;

    // This is a fixture function for tests. Fixtures are setup functions that
    // provide a fixed baseline upon which tests can reliably and repeatedly
    // execute. This particular fixture is providing a fixed seed for random
    // number generation in tests.
    #[fixture]
    fn seed() -> u64 {
        // We return a constant seed value of 2024. This seed can be used to
        // initialize a random number generator in a predictable way,
        // which is useful for testing code that involves random
        // behavior.
        RANDOM_SEED.0
    }

    // This is a fixture function that provides a NumberPair instance for testing
    #[fixture]
    fn number_pair_example() -> NumberPair {
        // We create a NumberPair with values 1 and 2
        NumberPair::new(1, 2)
    }

    // This test checks the Display implementation for NumberPair
    #[rstest]
    fn test_number_pair_display(_number_pair_example: NumberPair) {
        // We create a NumberPair with values 1 and 2
        let number_pair = NumberPair::new(1, 2);

        // We check that the Display implementation for NumberPair correctly converts
        // the NumberPair to a string
        assert_eq!(format!("{}", number_pair), "(1, 2)");
    }

    // This test checks the `default` constructor for NumberPair
    #[rstest]
    fn test_number_pair_default() {
        // We create a `NumberPair` using the `default` constructor
        let number_pair = NumberPair::default();

        // We check that the first number is less than 10 and greater than or equal to 1
        // This verifies that the `default` constructor correctly initializes the first
        // number
        assert!(number_pair.first() >= 1);

        assert!(number_pair.first() < 10);

        // We check that the second number is less than 10 and greater than
        // or equal to 1 This verifies that the `default` constructor correctly
        // initializes the second number
        assert!(number_pair.second() >= 1);

        assert!(number_pair.second() < 10);
    }

    // This test checks the `new` constructor for NumberPair
    #[rstest]
    fn test_number_pair_new() {
        // We create a `NumberPair` with values 1 and 2
        let number_pair = NumberPair::new(1, 2);

        // We check that the first number is 1 and the second number is 2
        // This verifies that the `new` constructor correctly initializes the numbers
        assert_eq!(number_pair.first(), 1);

        assert_eq!(number_pair.second(), 2);
    }

    // This test checks the `first` method of `NumberPair`
    #[rstest]
    fn test_number_pair_first() {
        // We create a `NumberPair` with values 1 and 2
        let number_pair = NumberPair::new(1, 2);

        // We check that the first method correctly returns the first number
        assert_eq!(number_pair.first(), 1);
    }

    // This test checks the `second` method of `NumberPair`
    #[rstest]
    fn test_number_pair_second() {
        // We create a `NumberPair` with values 1 and 2
        let number_pair = NumberPair::new(1, 2);

        // We check that the `second` method correctly returns the second number
        assert_eq!(number_pair.second(), 2);
    }

    #[rstest]
    fn test_number_pair_random() {
        // Generate a random `NumberPair` where both numbers are between 1 and 10
        // We use the `random` function of the `NumberPair` struct to generate a pair of
        // random numbers
        let number_pair = NumberPair::random(1, 10);

        // Check that the first number is less than or equal to 10
        // This assertion is to verify that the `random` function respects the upper
        // limit provided
        assert!(number_pair.first() <= 10);

        // Check that the first number is greater than or equal to 1
        // This assertion is to verify that the `random` function respects the lower
        // limit provided
        assert!(number_pair.first() >= 1);

        // Check that the second number is less than or equal to 10
        // Similar to the first assertion, this is to ensure that the `random` function
        // respects the upper limit for the second number
        assert!(number_pair.second() <= 10);

        // Check that the second number is greater than or equal to 1
        // Similar to the second assertion, this is to ensure that the `random` function
        // respects the lower limit for the second number
        assert!(number_pair.second() >= 1);
    }

    #[rstest]
    fn test_number_pair_random_with_seed(seed: u64) {
        // Generate a random NumberPair where both numbers are between 1 and 10, using a
        // specific seed
        let number_pair = NumberPair::random_seeded(1, 10, seed);

        // Check that the first number is less than or equal to 10
        // This ensures that the random number generation is respecting the upper limit
        assert!(number_pair.first() <= 10);

        // Check that the first number is greater than or equal to 1
        // This ensures that the random number generation is respecting the lower limit
        assert!(number_pair.first() >= 1);

        // Check that the first number is equal to 6
        // This is a specific check for the seeded random number generation.
        // Given a specific seed, the generated number should always be the same
        assert_eq!(number_pair.first(), 6);

        // Check that the second number is less than or equal to 10
        // This ensures that the random number generation is respecting the upper limit
        assert!(number_pair.second() <= 10);

        // Check that the second number is greater than or equal to 1
        // This ensures that the random number generation is respecting the lower limit
        assert!(number_pair.second() >= 1);

        // Check that the second number is equal to 6
        // This is a specific check for the seeded random number generation.
        // Given a specific seed, the generated number should always be the same
        assert_eq!(number_pair.second(), 9);
    }

    #[rstest]
    fn test_number_pair_random_with_seed_repeated(seed: u64) {
        // Generate a random NumberPair where both numbers are between 1 and 10, using a
        // specific seed
        let number_pair_1 = NumberPair::random_seeded(1, 10, seed);

        let number_pair_2 = NumberPair::random_seeded(1, 10, seed);

        // Check if number_pair_1 and number_pair_2 are equal
        // This is a specific check for the seeded random number generation.
        // Given a specific seed, the generated number should always be the same
        assert_eq!(number_pair_1, number_pair_2);
    }
}
