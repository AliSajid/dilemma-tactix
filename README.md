
# Repeated Prisoners Dilemma

[![Continuous Integration](https://github.com/AliSajid/dilemma-tactix/actions/workflows/ci.yaml/badge.svg)](https://github.com/AliSajid/dilemma-tactix/actions/workflows/ci.yaml)
[![Crates.io Version](https://img.shields.io/crates/v/dilemma-tactix?logo=rust&label=Crates.io%20Release)](https://crates.io/crates/dilemma-tactix)
[![GitHub tag (latest SemVer)](https://img.shields.io/github/v/release/AliSajid/dilemma-tactix?logo=github&label=Github%20Release)](https://github.com/AliSajid/dilemma-tactix/releases)
[![docs.rs](https://img.shields.io/docsrs/dilemma-tactix?logo=rust&label=docs.rs)](https://docs.rs/dilemma-tactix)
[![Crates.io License](https://img.shields.io/crates/l/dilemma-tactix?label=License)](https://github.com/AliSajid/dilemma-tactix/tree/main#license)
[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)

[![REUSE status](https://api.reuse.software/badge/github.com/AliSajid/dilemma-tactix)](https://api.reuse.software/info/github.com/AliSajid/dilemma-tactix)
[![CII Best Practices](https://img.shields.io/cii/percentage/9927?logo=securityscorecard&label=OpenSSF%20Best%20Practices)](https://www.bestpractices.dev/projects/9927)
[![OSS Lifecycle](https://img.shields.io/osslifecycle?file_url=https%3A%2F%2Fgithub.com%2FAliSajid%2Fdilemma-tactix%2Fblob%2Fmain%2FOSSMETADATA&label=OSS%20Lifecycle)](https://github.com/AliSajid/dilemma-tactix/blob/main/OSSMETADATA)
![OSSF-Scorecard Score](https://img.shields.io/ossf-scorecard/github.com/AliSajid/dilemma-tactix?label=OSSF%20Scorecard&logo=securityscorecard)
[![Codacy grade](https://img.shields.io/codacy/grade/293d6f6e3e5e4fadb1b88db426462f87?logo=codacy&label=Codacy%20Grade)](https://app.codacy.com/gh/AliSajid/dilemma-tactix/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)
[![CodeFactor Grade](https://img.shields.io/codefactor/grade/github/AliSajid/dilemma-tactix?logo=codefactor&label=CodeFactor%20Grade)](https://www.codefactor.io/repository/github/alisajid/dilemma-tactix)
[![Codecov](https://img.shields.io/codecov/c/github/AliSajid/dilemma-tactix?logo=codecov&label=Code%20Coverage)](https://codecov.io/gh/AliSajid/dilemma-tactix)
[![Security Audit](https://github.com/AliSajid/dilemma-tactix/actions/workflows/audit.yaml/badge.svg)](https://github.com/AliSajid/dilemma-tactix/actions/workflows/audit.yaml)

![Crates.io MSRV](https://img.shields.io/crates/msrv/dilemma-tactix?label=MSRV)
![Libraries.io SourceRank](https://img.shields.io/librariesio/sourcerank/cargo/dilemma-tactix)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/alisajid/dilemma-tactix/latest)
![GitHub Created At](https://img.shields.io/github/created-at/AliSajid/dilemma-tactix?label=Created)

[![Code of Conduct: Contributor Covenant](https://img.shields.io/badge/Code_of_Conduct-Contributor_Covenant_v2.1-14cc21)](https://github.com/EthicalSource/contributor_covenant)

A simple project to simulate prisoners dilemma and multiple strategies

## Builds

|         | Stable | Beta| Nightly| MSRV (1.74.1)|
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Linux   | ![Ubuntu x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/2142caf9bfe6fc8cdc7d1b8ccd72ce09/raw/ubuntu-stable.json)   | ![Ubuntu x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/ubuntu-beta.json)   | ![Ubuntu x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/ubuntu-nightly.json)   | ![Ubuntu x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/ubuntu-msrv.json)   |
| Windows | ![Windows x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/2142caf9bfe6fc8cdc7d1b8ccd72ce09/raw/windows-stable.json) | ![Windows x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/windows-beta.json) | ![Windows x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/windows-nightly.json) | ![Windows x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/windows-msrv.json) |
| macOS   | ![macOS x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/2142caf9bfe6fc8cdc7d1b8ccd72ce09/raw/macos-stable.json)     | ![macOS x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/macos-beta.json)     | ![macOS x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/macos-nightly.json)     | ![macOS x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/d52f912107d7609656370db9d741596c/raw/macos-msrv.json)     |

## Current Status

The current status of the project is
