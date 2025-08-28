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
// extern crate intel_mkl_src;

use crate::marian_mt::{run_model, Args};
use crate::util::{html_wrapper, parse_args};

const HELP_TEXT: &str = r#"usage: gd-translate [OPTIONS]

Translate text from Japanese to English using Helsinki-NLP/opus-mt-ja-en

OPTIONS
  --sentence TEXT       The text to translate (required)
  --spoiler             Black out the translation with a spoiler box
  -h, --help        Print this help screen

EXAMPLES
gd-tools translate --sentence "こんにちはお元気ですか" --spoiler
"#;

const CSS_STYLE: &str = r#"
.spoiler {
  background-color: black;
  padding: 6px;
  width: fit-content;
}
.spoiler:hover {
  background-color: white;
}
.error {
  color: red;
  font-weight: bold;
}
"#;

fn print_help() {
    println!("{}", HELP_TEXT);
}

pub fn translate_text(args: &[String]) {
    if args.is_empty()
        || args.contains(&String::from("--help"))
        || args.contains(&String::from("-h"))
    {
        print_help();
        return;
    }

    let parsed_args = parse_args(args);

    let text = match parsed_args.get("sentence") {
        Some(t) => t,
        None => {
            eprintln!("Error: Missing required argument --sentence");
            print_help();
            return;
        }
    };

    let spoiler = parsed_args.get("spoiler").is_some();

    let arguments = Args {
        cpu: true,
        text: text.to_string(),
    };

    let translated_text = run_model(arguments);
    match translated_text {
        Ok(result) => {
            let html = format!(
                r#"<div{}>{}</div>"#,
                if spoiler { " class=\"spoiler\"" } else { "" },
                result
            );
            println!("{}", html_wrapper(&html, Some(CSS_STYLE)));
        }
        Err(e) => {
            let html = format!(r#"<div class="error">{}</div>"#, e.to_string());
            println!("{}", html_wrapper(&html, Some(CSS_STYLE)));
        }
    }
    return;
}
