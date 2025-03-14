# sasquatchdev/squarewords

A word-square finder/solver written in rust.

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Installation & Usage](#installation--usage)
- [Configuration](#configuration)

## Introduction

I made this project, following [this](https://www.youtube.com/watch?v=zWIsnrxL-Zc) youtube video by [CodeParade](https://www.youtube.com/@CodeParade). I did not use any of his code, but it helped me understand the topic.

## Features

- Find solutions for word-squares of any size
- _(planned)_ Solve partial word-squares of any size
- _(planned)_ Configuration via `Words.toml`

## Installation & Usage

To use this, simply make sure you have [Rust](https://rustup.rs/) and [Git](https://git-scm.com/) installed, and clone this repository.

```bash
git clone https://github.com/sasquatchdev/squarewords.git
```

Next, [configure](#configuration) the tool to your liking, and run it.

```bash
cargo run --quiet
```

## Configuration

Currently, to configure this, you'll have to edit the Constants at the begining of the `main.rs` file. You have three options:

1. `const SIZE: usize` - the size of the word square (both width and heiht)

2. `const TOP_N: u64` - the lower it gets, the more popular used words have to be (0 means no restriction)

3. `const UNIQUE: bool` - if words can appear multiple times in the same square

_In the future, I will add a `Words.toml` configuration file which will replace this._