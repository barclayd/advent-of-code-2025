# Advent of Code 2025

[![ci: passing](https://img.shields.io/badge/ci-passing-brightgreen?style=for-the-badge)](https://github.com/barclayd/advent-of-code-2024/actions)
&nbsp;
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=orange)](https://www.rust-lang.org/)

Solutions for [Advent of Code 2025](https://adventofcode.com/2025) written in Rust 🦀

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition or later)

## Getting Started

```bash
git clone https://github.com/barclayd/advent-of-code-2025.git
cd advent-of-code-2025
cargo build
cargo test
```

## Scripts

### Start a new day

Prerequisites: 

* Get your AoC Session Cookie

This can be done by copying the value by going to https://adventofcode.com/ and opening Dev Tools.
Chrome: `Application` => `Cookies` => `https://adventofcode.com/` => `session: <value>`
Copy the value and paste it into a newly created `.env`, based on `.env.local`

```sh
brew install pup
./scripts/new-day.sh
```

* This will generate a new folder with a template ready to be worked on, including test setup and a blank a `test.txt`.

`test.txt` requires manual copying and pasting from the puzzle html at present

### Get input

```shell
./scripts/puzzle-input.sh
```

This command auto generates your `input.txt` file and places it in the latest day folder.

For example, if you are working on a solution for Day 4, it will place it in `/day-04/input.txt`, ready to be used in your solution.

It is to be run when you have understood the puzzle and your tests locally for `test.txt` are passing.

## Project Structure

```
advent-of-code-2025/
├── day-01/
├── day-02/
├── day-03/
...
└── README.md
```

## Continuous Integration

This project uses GitHub Actions for continuous integration. The workflow:

- Runs on every push to `main`and pull request against `main`
- Tests solution for every day

The workflow configuration can be found in `.github/workflows/ci.yml`.
These run in a parallelized matrix.

## Progress (18/24 ⭐️)

| Day | Challenge                                                  | Stars |
|-----|------------------------------------------------------------|-------|
| 1   | [Secret Entrance](https://adventofcode.com/2025/day/1)     | ⭐️⭐️  |
| 2   | [Gift Shop ](https://adventofcode.com/2025/day/2)          | ⭐️⭐️  |
| 3   | [Lobby](https://adventofcode.com/2025/day/3)               | ⭐️⭐️  |
| 4   | [Printing Department](https://adventofcode.com/2025/day/4) | ⭐⭐    |
| 5   | [Cafeteria](https://adventofcode.com/2025/day/5)           | ⭐⭐    |
| 6   | [Trash Compactor](https://adventofcode.com/2025/day/6)     | ⭐⭐    |
| 7   | [Laboratories](https://adventofcode.com/2025/day/7)        | ⭐⭐    |
| 8   | [Playground](https://adventofcode.com/2025/day/8)          | ⭐⭐    |
| 9   | [Movie Theatre](https://adventofcode.com/2025/day/9)       | ⭐⭐    |
| 10  | [Factory](https://adventofcode.com/2025/day/10)            |       |
