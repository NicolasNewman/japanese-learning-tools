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

use crate::util::{parse_args, html_wrapper};
use crate::marian_mt::{run_model, Args};

const HELP_TEXT: &str = r#"usage: gd-translate [OPTIONS]

Translate text from Japanese to English using Helsinki-NLP/opus-mt-ja-en

OPTIONS
  --text TEXT       The text to translate (required)
  -h, --help        Print this help screen

EXAMPLES
gd-translate --text "こんにちはお元気ですか"
"#;

const CSS_STYLE: &str = r#"
.translation-container {
  padding: 15px;
  max-width: 100%;
}
.translation-text {
  margin-bottom: 10px;
  font-size: 1.1em;
}
.source-lang, .target-lang {
  color: #888;
  font-size: 0.9em;
  margin-bottom: 5px;
}
.translation-result {
  padding: 10px;
  border-left: 3px solid #4a90e2;
  background-color: #f8f9fa;
  margin-top: 10px;
  font-size: 1.2em;
}
"#;

fn print_help() {
    println!("{}", HELP_TEXT);
}

pub fn translate_text(args: &[String]) {
    if args.is_empty() || args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        print_help();
        return;
    }
    
    let parsed_args = parse_args(args);
    
    // Get text to translate
    let text = match parsed_args.get("text") {
        Some(t) => t,
        None => {
            eprintln!("Error: Missing required argument --text");
            print_help();
            return;
        }
    };

    let arguments = Args {
        cpu: true,
        text: text.to_string(),
    };

    let translated_text = run_model(arguments);
    match translated_text {
        Ok(result) => {
          let html = format!(
              r#"<div class="translation-container">
                  <div class="translation-result">{}</div>
              </div>"#,
              result
          );
          println!("{}", html_wrapper(&html, Some(CSS_STYLE)));
        }
        Err(e) => {
            let html = format!(
              r#"<div class="translation-container">
                  <div class="translation-error">{}</div>
              </div>"#,
              e.to_string()
          );
          println!("{}", html_wrapper(&html, Some(CSS_STYLE)));
        }
    }
   return;
}
