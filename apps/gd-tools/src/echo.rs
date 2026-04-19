/*
 *  gd-tools - a set of programs to enhance goldendict for immersion learning.
 *  Copyright (C) 2023-2025 Ajatt-Tools
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::util::{parse_args, GdToolsError, Result};
use std::process;

// In C++ CharByteLen::THREE * 5UL is used, which is 3 bytes per character * 5 characters
const DEFAULT_LEN: usize = 3 * 5;
const HELP_TEXT: &str = r#"usage: gd-tools echo [OPTIONS]

Echo input back to GoldenDict as HTML with the KanjiStrokeOrders font applied.

OPTIONS
  --max-len NUMBER  maximum length of the input string (bytes).
  --font-size SIZE  font size (with unit).
  --font-family     font family (name).
  --word WORD       text to print.

EXAMPLES
  gd-tools echo --max-len 5 --font-size 10rem --word "書"
  gd-tools echo --max-len 3 --font-size 120px --word "薔薇"
"#;

struct StrokeOrderParams {
    gd_word: String,
    font_size: String,
    font_family: String,
    max_len: usize,
}

impl StrokeOrderParams {
    fn new() -> Self {
        Self {
            gd_word: String::new(),
            font_size: String::from("10rem"),
            font_family: String::from("KanjiStrokeOrders"),
            max_len: DEFAULT_LEN,
        }
    }

    fn assign(&mut self, key: &str, value: &str) {
        if key == "max-len" {
            if let Ok(len) = value.parse::<usize>() {
                self.max_len = len;
            }
        } else if key == "font-size" {
            self.font_size = value.to_string();
        } else if key == "font-family" {
            self.font_family = value.to_string();
        } else if key == "word" {
            self.gd_word = value.to_string();
        }
    }
}

fn print_css(params: &StrokeOrderParams) {
    let pid = process::id();
    let css = r#"
  <style>
  .gd_echo_{} {{
      font-size: {};
      font-family: "{}";
  }}
  </style>
  "#;
    println!(
        "{}",
        css.replace("{}", &pid.to_string())
            .replace("{}", &params.font_size)
            .replace("{}", &params.font_family)
    );
}

fn print_with_stroke_order(params: &StrokeOrderParams) {
    if params.gd_word.len() <= params.max_len {
        let pid = process::id();
        println!("<div class=\"gd_echo_{}\">{}</div>", pid, params.gd_word);
        print_css(params);
    }
}

fn parse_params(args: &[String]) -> Result<StrokeOrderParams> {
    let mut params = StrokeOrderParams::new();

    let parsed_args = parse_args(args);

    for (key, value) in parsed_args {
        params.assign(&key, &value);
    }

    if params.gd_word.is_empty() {
        return Err(GdToolsError::MissingArgument("--word".into()));
    }

    Ok(params)
}

pub fn show_stroke_order(args: &[String]) {
    if args.is_empty()
        || args.contains(&String::from("--help"))
        || args.contains(&String::from("-h"))
    {
        println!("{}", HELP_TEXT);
        return;
    }

    match parse_params(args) {
        Ok(params) => {
            print_with_stroke_order(&params);
        }
        Err(e) => {
            eprintln!("{}", e);
            println!("{}", HELP_TEXT);
        }
    }
}

// Since the C++ implementation only has one function, we'll map both public functions to the same code
pub fn show_handwritten(args: &[String]) {
    show_stroke_order(args);
}
