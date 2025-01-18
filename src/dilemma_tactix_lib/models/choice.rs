// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// Import the fmt module for formatting
use std::fmt;

// Define an enum called Choice with two variants: Atlantis and Olympus
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Choice {
    Atlantis,
    Olympus,
}

// Implement methods for the Choice enum
impl Choice {
    // Define a method to return the Atlantis variant of Choice
    // Example: let my_choice = Choice::atlantis(); // my_choice is now
    // Choice::Atlantis
    #[must_use]
    pub const fn atlantis() -> Self {
        Self::Atlantis
    }

    // Define a method to return the Olympus variant of Choice
    // Example: let my_choice = Choice::olympus(); // my_choice is now
    // Choice::Olympus
    #[must_use]
    pub const fn olympus() -> Self {
        Self::Olympus
    }
}

// Implement the Display trait for the Choice enum
// This allows a Choice to be formatted as a string using the "{}" format
// specifier
impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write the string representation of the Choice to the provided formatter
        // Example: println!("{}", Choice::Atlantis); // Prints: Atlantis
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_choice_to_string() {
        assert_eq!(Choice::Atlantis.to_string(), "Atlantis");

        assert_eq!(Choice::Olympus.to_string(), "Olympus");
    }

    #[test]
    fn test_choice_atlantis() {
        assert_eq!(Choice::atlantis(), Choice::Atlantis);
    }

    #[test]
    fn test_choice_olympus() {
        assert_eq!(Choice::olympus(), Choice::Olympus);
    }

    #[test]
    fn test_choice_display() {
        assert_eq!(format!("{}", Choice::Atlantis), "Atlantis");

        assert_eq!(format!("{}", Choice::Olympus), "Olympus");
    }
}
