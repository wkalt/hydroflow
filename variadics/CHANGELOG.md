# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.0.3 (2024-01-29)

### Chore

 - <csr-id-1b555e57c8c812bed4d6495d2960cbf77fb0b3ef/> manually set lockstep-versioned crates (and `lattices`) to version `0.5.1`
   Setting manually since
   https://github.com/frewsxcv/rust-crates-index/issues/159 is messing with
   smart-release

### Refactor

 - <csr-id-7e65a08711775656e435e854777c5f089dd31a05/> Improvements prepping for release
   - Adds the "spread"/"splat" `...` syntax to the three variadics macros.
   - Adds `#[sealed]` traits.
   - Adds testing of error messages.
   - Improves docs: `README.md` and Rust docs.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 37 calendar days.
 - 253 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#974](https://github.com/hydro-project/hydroflow/issues/974)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#974](https://github.com/hydro-project/hydroflow/issues/974)**
    - Improvements prepping for release ([`7e65a08`](https://github.com/hydro-project/hydroflow/commit/7e65a08711775656e435e854777c5f089dd31a05))
 * **Uncategorized**
    - Manually set lockstep-versioned crates (and `lattices`) to version `0.5.1` ([`1b555e5`](https://github.com/hydro-project/hydroflow/commit/1b555e57c8c812bed4d6495d2960cbf77fb0b3ef))
</details>

## 0.0.2 (2023-05-21)

<csr-id-5a3c2949653685de1e33cf7412057a70880283df/>

### Style

 - <csr-id-5a3c2949653685de1e33cf7412057a70880283df/> rustfmt format code comments

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 24 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#660](https://github.com/hydro-project/hydroflow/issues/660)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#660](https://github.com/hydro-project/hydroflow/issues/660)**
    - Rustfmt format code comments ([`5a3c294`](https://github.com/hydro-project/hydroflow/commit/5a3c2949653685de1e33cf7412057a70880283df))
 * **Uncategorized**
    - Release hydroflow_cli_integration v0.0.1, hydroflow_lang v0.0.1, hydroflow_datalog_core v0.0.1, hydroflow_datalog v0.0.1, hydroflow_macro v0.0.1, lattices v0.1.0, variadics v0.0.2, pusherator v0.0.1, hydroflow v0.0.2 ([`809395a`](https://github.com/hydro-project/hydroflow/commit/809395acddb78949d7a2bf036e1a94972f23b1ad))
</details>

## 0.0.1 (2023-04-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 133 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#617](https://github.com/hydro-project/hydroflow/issues/617)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#617](https://github.com/hydro-project/hydroflow/issues/617)**
    - Update `Cargo.toml`s for publishing ([`a78ff9a`](https://github.com/hydro-project/hydroflow/commit/a78ff9aace6771787c2b72aad83be6ad8d49a828))
 * **Uncategorized**
    - Setup release workflow ([`108d0e9`](https://github.com/hydro-project/hydroflow/commit/108d0e933a08b183c4dadf8c3499e4946696e263))
    - Rename variadics/tuple_list macros ([`91d37b0`](https://github.com/hydro-project/hydroflow/commit/91d37b022b1cd0ed590765c40ef43244027c8035))
    - Rename pkg `type_list` -> `variadics` ([`50e7361`](https://github.com/hydro-project/hydroflow/commit/50e7361709cd34fd0e1cbf0c9a9f79343ee9c2e2))
</details>

