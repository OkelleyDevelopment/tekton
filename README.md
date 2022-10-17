# Tekton

![Workflow](https://github.com/OkelleyDevelopment/tekton/actions/workflows/rust.yml/badge.svg)

Author(s): Nicholas O'Kelley

Date: 2022-08-28
Last Modified: 2022-10-16

NOTE: This is still a tool in alpha and might rapidly change.

## Motivation

I needed a **blazingly fast** tool to both convert and sort snippets for 
the [`friendly-snippet`](https://github.com/rafamadriz/friendly-snippets) project as 
doing this by hand was very time consuming.

## Installation and Execution

- `cargo install tekton`

To convert: 

- `tekton convert <input> <output>`

The current mappings support bidirectional conversion between Snipmate (`.snippet`) and JSON

To sort: 
- `tekton sort <INPUT_FILENAME>`

## Acknowledgements

- My impatience for doing this by hand and desire to automate everything