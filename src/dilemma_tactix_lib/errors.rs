// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt;

#[derive(Debug)]
pub enum BuilderError {
    InvalidOptionSpecified(String),
    InvalidOptionValueSpecified(String),
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidOptionSpecified(ref s) => {
                write!(f, "Invalid option specified: {s}")
            }
            Self::InvalidOptionValueSpecified(ref s) => {
                write!(f, "Invalid option value specified: {s}")
            }
        }
    }
}

impl std::error::Error for BuilderError {}
