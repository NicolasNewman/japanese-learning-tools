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

use crate::util::{parse_args, html_wrapper, GdToolsError, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[cfg(feature = "marisa")]
use trie_rs::TrieBuilder;

const HELP_TEXT: &str = r#"usage: gd-marisa [OPTIONS]

Split Japanese text using the MARISA trie dictionary.

OPTIONS
  --text TEXT     The text to split (required)
  -h, --help      Print this help screen

EXAMPLES
gd-marisa --text "日本語の勉強が好きです"
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
}
.segment:hover {
  background-color: #f0f0f0;
  cursor: pointer;
}
"#;

fn print_help() {
    println!("{}", HELP_TEXT);
}

#[cfg(feature = "marisa")]
pub fn load_dictionary(dict_path: &str) -> Result<trie_rs::Trie<u8>> {
    let path = Path::new(dict_path);
    if !path.exists() {
        return Err(GdToolsError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Dictionary file not found: {}", dict_path)
        )));
    }
    
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let mut builder = TrieBuilder::new();
    
    for line in reader.lines() {
        if let Ok(word) = line {
            builder.push(word);
        }
    }
    
    Ok(builder.build())
}

#[cfg(feature = "marisa")]
pub fn segment_text(text: &str, trie: &trie_rs::Trie<u8>) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;
    
    while i < text.len() {
        let mut longest_match = "";
        
        // Try to find the longest match starting at position i
        for j in (i + 1)..=text.len() {
            let substring = &text[i..j];
            if trie.exact_match(substring) {
                longest_match = substring;
            }
        }
        
        if longest_match.is_empty() {
            // If no match found, use a single character
            let ch_len = text[i..].chars().next().map_or(1, |c| c.len_utf8());
            result.push(text[i..i+ch_len].to_string());
            i += ch_len;
        } else {
            result.push(longest_match.to_string());
            i += longest_match.len();
        }
    }
    
    result
}

#[cfg(not(feature = "marisa"))]
fn segment_text_fallback(text: &str) -> Vec<String> {
    // Simple character-by-character fallback when marisa feature is disabled
    text.chars().map(|c| c.to_string()).collect()
}

pub fn split_marisa(args: &[String]) {
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
    
    // Get dictionary path
    let dict_path = parsed_args.get("path-to-dic")
        .cloned()
        .unwrap_or_else(|| {
            // Default dictionary path
            let mut path = std::env::current_exe().unwrap_or_default();
            path.pop(); // Remove executable name
            path.push("res");
            path.push("marisa_words.dic");
            path.to_string_lossy().to_string()
        });
    
    #[cfg(feature = "marisa")]
    let segments = match load_dictionary(&dict_path) {
        Ok(trie) => segment_text(text, &trie),
        Err(e) => {
            eprintln!("Error loading dictionary: {}", e);
            // Fallback to simple character-by-character segmentation
            text.chars().map(|c| c.to_string()).collect()
        }
    };
    
    #[cfg(not(feature = "marisa"))]
    let segments = segment_text_fallback(text);
    
    let segments_html = segments
        .into_iter()
        .map(|seg| format!("<span class=\"segment\">{}</span>", seg))
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

/// Convert a MARISA trie dictionary to a format usable by trie-rs
/// This is useful for migrating from the original C++ implementation
pub fn convert_marisa_dictionary(input_path: &str, output_path: &str) -> Result<()> {
    use std::io::Write;
    
    let input = Path::new(input_path);
    if !input.exists() {
        return Err(GdToolsError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Input dictionary file not found: {}", input_path)
        )));
    }
    
    // Open the raw words file (should be newline-separated words)
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    
    // Create the output file
    let mut output = File::create(output_path)?;
    
    // Copy all lines from input to output
    for line in reader.lines() {
        if let Ok(word) = line {
            writeln!(output, "{}", word)?;
        }
    }
    
    Ok(())
}
