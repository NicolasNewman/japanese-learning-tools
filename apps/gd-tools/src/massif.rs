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

use crate::util::{parse_args, html_wrapper};

const HELP_TEXT: &str = r#"usage: gd-massif [OPTIONS]

Search for words on Massif dictionary.

OPTIONS
  --word WORD      The word to search for (required)
  --max-results N  Maximum number of results to return (default: 10)
  -h, --help       Print this help screen

EXAMPLES
gd-massif --word "日本語"
gd-massif --word "勉強" --max-results 5
"#;

const CSS_STYLE: &str = r#"
.massif-results {
  padding: 10px;
}
.result-item {
  margin-bottom: 15px;
  padding: 10px;
  border-left: 3px solid #4a90e2;
  background-color: #f8f9fa;
}
.result-title {
  font-size: 1.2em;
  font-weight: bold;
  margin-bottom: 5px;
}
.result-reading {
  color: #666;
  margin-bottom: 5px;
}
.result-definition {
  margin-top: 5px;
}
"#;

fn print_help() {
    println!("{}", HELP_TEXT);
}

pub fn search_massif(args: &[String]) {
    if args.is_empty() || args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        print_help();
        return;
    }
    
    let parsed_args = parse_args(args);
    
    // Get word to search for
    let word = match parsed_args.get("word") {
        Some(w) => w,
        None => {
            eprintln!("Error: Missing required argument --word");
            print_help();
            return;
        }
    };
    
    // Get maximum results
    let max_results = parsed_args.get("max-results")
        .and_then(|m| m.parse::<usize>().ok())
        .unwrap_or(10);
    
    // TODO: Implement actual Massif dictionary search
    
    // For now, just return a placeholder result
    let html = format!(
        r#"<div class="massif-results">
            <div class="result-item">
                <div class="result-title">{}</div>
                <div class="result-reading">にほんご</div>
                <div class="result-definition">Japanese language</div>
            </div>
            <div class="result-item">
                <div class="result-title">日本</div>
                <div class="result-reading">にほん</div>
                <div class="result-definition">Japan</div>
            </div>
        </div>"#,
        word
    );
    
    println!("{}", html_wrapper(&html, Some(CSS_STYLE)));
}
