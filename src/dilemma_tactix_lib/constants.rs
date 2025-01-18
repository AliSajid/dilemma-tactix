// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// A constant used to seed the random number generator.
///
/// This is a seed used to generate random numbers. It is used to ensure that
/// the random number generator is deterministic when testing.
#[cfg(test)]
pub const RANDOM_SEED: (u64, u64, u64, u64) = (2024, 2023, 2022, 2021);
