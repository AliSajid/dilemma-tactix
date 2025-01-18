// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

struct Cli {
    minimum: Option<usize>,
    maximum: Option<usize>,
    score_aa: Option<[usize; 2]>,
    score_ab: Option<[usize; 2]>,
    score_ba: Option<[usize; 2]>,
    score_bb: Option<[usize; 2]>,
    randomize: bool,
}
