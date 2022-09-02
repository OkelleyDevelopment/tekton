# Tekton

![Workflow](https://github.com/OkelleyDevelopment/tekton/actions/workflows/rust.yml/badge.svg)

Author(s): Nicholas O'Kelley

Date: 2022-08-28

NOTE: This is still a tool in alpha and might rapidly change.

## Motivation

I needed a **blazingly fast** tool to speed up my ability to convert
`.snippets` files to `.json` for the
[`friendly-snippet`](https://github.com/rafamadriz/friendly-snippets) project. The goal
of this project is now to allow various snippet mappings to coexist.

## Project Execution

### Manual

1. Clone this repository
2. Move into the project directory
3. Run the following command:

```
cargo run <.snippet file to convert> <path for JSON output (include .json)>

```

## Future Goals

These are current ideas I've got in mind, but there is currently not a
definitive road map for which would finished first.

- Provide options for snippets
- A better cli interface
- Allow users to switch JSON back to the `.snippet` format
- Edit descriptions?

## Known Bugs

- None at this time.

## Acknowledgements

- My impatience for doing this by hand
