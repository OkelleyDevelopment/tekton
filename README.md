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

## Installation and Execution

- `cargo install tekton`

- `tekton <snippet file to convert> <json output file>`


## Future Goals

These are current ideas I've got in mind, but there is currently not a
definitive road map for which would finished first.

- Provide options for snippets
- A better cli interface
- Allow users to switch JSON back to the `.snippet` format
- Edit descriptions?


## Acknowledgements

- My impatience for doing this by hand
