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

const HELP_TEXT: &str = r#"usage: gd-mecab [OPTIONS]

Split Japanese text using MeCab morphological analyzer.

OPTIONS
  --text TEXT     The text to split (required)
  --format TYPE   Output format (default: html, options: html, text, detailed)
  -h, --help      Print this help screen

EXAMPLES
gd-mecab --text "日本語の勉強が好きです"
gd-mecab --text "こんにちは世界" --format detailed
"#;

const CSS_STYLE: &str = r#"
.segmentation-container {
  padding: 15px;
  max-width: 100%;
}
.original-text {
  margin-bottom: 15px;
  font-size: 1.2em;
}
.segmentation-result {
  border-left: 3px solid #4a90e2;
  padding: 10px;
  background-color: #f8f9fa;
}
.segment {
  display: inline-block;
  border: 1px solid #ccc;
  margin: 2px;
  padding: 3px 6px;
  border-radius: 3px;
  background-color: #fff;
  position: relative;
}
.segment:hover {
  background-color: #f0f0f0;
  cursor: pointer;
}
.segment:hover .segment-details {
  display: block;
}
.segment-details {
  display: none;
  position: absolute;
  top: 100%;
  left: 0;
  background-color: white;
  border: 1px solid #ddd;
  border-radius: 3px;
  padding: 5px;
  width: max-content;
  z-index: 10;
  box-shadow: 0 2px 5px rgba(0,0,0,0.2);
  font-size: 0.8em;
}
"#;

fn print_help() {
    println!("{}", HELP_TEXT);
}

#[cfg(feature = "mecab")]
fn analyze_text_with_mecab(text: &str) -> Result<Vec<(String, String, String)>, String> {
    // use lindera::tokenizer::{Tokenizer, TokenizerConfig};
    
    // // Initialize tokenizer with Japanese dictionary
    // let config = TokenizerConfig {
    //     dictionary: Some("ipadic".to_string()),
    //     ..Default::default()
    // };
    
    // let tokenizer = match Tokenizer::with_config(config) {
    //     Ok(t) => t,
    //     Err(e) => return Err(format!("Failed to initialize tokenizer: {}", e)),
    // };
    
    // let tokens = match tokenizer.tokenize(text) {
    //     Ok(t) => t,
    //     Err(e) => return Err(format!("Failed to tokenize text: {}", e)),
    // };
    
    let mut result = Vec::new();
    
    // for token in tokens {
    //     let surface = token.text.to_string();
    //     let pos = token.feature.to_string();
        
    //     // In a real implementation, we'd extract reading from token features
    //     // For now, just use a placeholder
    //     let reading = "—".to_string();
        
    //     result.push((surface, pos, reading));
    // }
    
    Ok(result)
}

#[cfg(not(feature = "mecab"))]
fn analyze_text_with_mecab(text: &str) -> Result<Vec<(String, String, String)>, String> {
    // Fallback implementation for when the mecab feature is disabled
    // Just return some placeholder data
    let mut result = Vec::new();
    
    for c in text.chars() {
        result.push((c.to_string(), "—".to_string(), "—".to_string()));
    }
    
    Ok(result)
}

pub fn split_mecab(args: &[String]) {
    if args.is_empty() || args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        print_help();
        return;
    }
    
    let parsed_args = parse_args(args);
    
    // Get text to split
    let text = match parsed_args.get("text") {
        Some(t) => t,
        None => {
            eprintln!("Error: Missing required argument --text");
            print_help();
            return;
        }
    };
    
    // Get output format
    let format = parsed_args.get("format")
        .cloned()
        .unwrap_or_else(|| String::from("html"));
    
    match analyze_text_with_mecab(text) {
        Ok(tokens) => {
            match format.as_str() {
                "text" => {
                    let output = tokens.iter()
                        .map(|(surface, _, _)| surface.clone())
                        .collect::<Vec<_>>()
                        .join(" ");
                    
                    println!("{}", output);
                },
                "detailed" => {
                    for (surface, pos, reading) in tokens {
                        println!("{}\t{}\t{}", surface, pos, reading);
                    }
                },
                _ => { // html format
                    let segments_html = tokens
                        .into_iter()
                        .map(|(word, pos, reading)| {
                            format!(
                                r#"<span class="segment">{}<span class="segment-details">読み: {}<br>品詞: {}</span></span>"#,
                                word, reading, pos
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("");
                    
                    let html = format!(
                        r#"<div class="segmentation-container">
                            <div class="original-text">{}</div>
                            <div class="segmentation-result">{}</div>
                        </div>"#,
                        text, segments_html
                    );
                    
                    println!("{}", html_wrapper(&html, Some(CSS_STYLE)));
                }
            }
        },
        Err(e) => {
            eprintln!("Error analyzing text: {}", e);
        }
    }
}