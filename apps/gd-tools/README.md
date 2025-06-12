# gd-tools

A set of programs to enhance GoldenDict for immersion learning, written in Rust.

## Overview

gd-tools provides several utilities for language immersion learning with GoldenDict:

- **ankisearch**: Search for words in your Anki collection
- **massif**: Search for words on Massif dictionary
- **images**: Search for images on Bing
- **translate**: Translate text using translation libraries
- **marisa**: Split search strings using MARISA trie
- **mecab**: Split search strings using MeCab
- **strokeorder**: Show stroke order of characters
- **handwritten**: Display handwritten form of characters

## Installation

### Dependencies

- Rust toolchain (cargo, rustc)
- MeCab (for word segmentation)
- MARISA (for trie-based word segmentation)
- ImageMagick (for stroke order and handwritten display)
- Fonts:
  - KanjiStrokeOrders font
  - ArmedLemon font

### Building from source

```bash
cargo build --release
```

## Usage

```bash
gd-tools ACTION [OPTIONS]
```

Where ACTION is one of:

- ankisearch
- massif
- images
- translate
- marisa
- mecab
- strokeorder
- handwritten

Each command has its own options. Use `gd-tools COMMAND --help` for details.

### Examples

Search for a word in Anki:
```bash
gd-tools ankisearch --field-name VocabKanji --word "日本語"
```

Display stroke order of characters:
```bash
gd-tools strokeorder --text "漢字"
```

Search for images:
```bash
gd-tools images --query "東京"
```

## License

This project is licensed under the GNU General Public License v3.0.
