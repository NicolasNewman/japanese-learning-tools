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

use crate::util::{GdToolsError, Result, parse_args, html_wrapper, escape_html};
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::time::Duration;

const ANKICONNECT_ADDR: &str = "127.0.0.1:8765";
const TIMEOUT_SECONDS: u64 = 3;
const HELP_TEXT: &str = r#"usage: gd-ankisearch [OPTIONS]

Search your Anki collection and output Note Ids that match query.

OPTIONS
  --field-name NAME    optional field to limit search to.
  --deck-name NAME     optional deck to limit search to.
  --show-fields F1,F2  optional comma-separated list of fields to show.
  --word WORD          required search term

EXAMPLES
gd-ankisearch --field-name VocabKanji --word %GDWORD%
gd-ankisearch --deck-name Mining --word %GDWORD%
"#;

const CSS_STYLE: &str = r#"
.gd-table-wrap {
  max-width: 100%;
  overflow-x: auto;
}
table {
  border-collapse: collapse;
  width: 100%;
  margin: 10px 0;
}
th, td {
  border: 1px solid #ddd;
  padding: 8px;
  text-align: left;
}
th {
  background-color: #f2f2f2;
}
tr:nth-child(even) {
  background-color: #f9f9f9;
}
tr:hover {
  background-color: #f1f1f1;
}
"#;

/// Execute AnkiConnect API call
fn call_ankiconnect(action: &str, params: Value) -> Result<Value> {
    let client = Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECONDS))
        .build()?;
    
    let request = json!({
        "action": action,
        "version": 6,
        "params": params
    });
    
    let response = client.post(format!("http://{}", ANKICONNECT_ADDR))
        .json(&request)
        .send()?
        .json::<Value>()?;
    
    if response["error"].is_null() {
        Ok(response["result"].clone())
    } else {
        Err(GdToolsError::ServiceUnavailable(format!(
            "AnkiConnect error: {}", 
            response["error"].as_str().unwrap_or("Unknown error")
        )))
    }
}

fn print_help() {
    println!("{}", HELP_TEXT);
}

fn format_anki_results(notes: Value, show_fields: &[&str]) -> String {
    let mut html = String::from("<div class=\"gd-table-wrap\"><table>");
    
    // Add table header
    html.push_str("<tr>");
    for &field in show_fields {
        html.push_str(&format!("<th>{}</th>", escape_html(field)));
    }
    html.push_str("</tr>");
    
    // Add note data
    if let Some(notes_array) = notes.as_array() {
        for note in notes_array {
            html.push_str("<tr>");
            
            for &field in show_fields {
                let content = note["fields"][field]["value"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
                
                html.push_str(&format!("<td>{}</td>", content));
            }
            
            html.push_str("</tr>");
        }
    }
    
    html.push_str("</table></div>");
    html_wrapper(&html, Some(CSS_STYLE))
}

pub fn search_anki_cards(args: &[String]) {
    if args.is_empty() || args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        print_help();
        return;
    }
    
    let parsed_args = parse_args(args);
    
    // Get search word
    let word = match parsed_args.get("word") {
        Some(w) => w,
        None => {
            eprintln!("Error: Missing required argument --word");
            print_help();
            return;
        }
    };
    
    // Build search query
    let mut query = String::new();
    
    if let Some(field_name) = parsed_args.get("field-name") {
        query.push_str(&format!("\"{}:{}\"", field_name, word));
    } else {
        query.push_str(word);
    }
    
    // Deck restriction
    let deck = parsed_args.get("deck-name").cloned();
    
    // Fields to show
    let show_fields = parsed_args.get("show-fields")
        .map(|s| s.split(',').collect::<Vec<_>>())
        .unwrap_or_else(|| vec!["Front", "Back"]);
    
    // Build AnkiConnect parameters
    let mut params = json!({
        "query": query
    });
    
    if let Some(deck_name) = deck {
        params["deck"] = json!(deck_name);
    }
    
    match call_ankiconnect("findNotes", params) {
        Ok(note_ids) => {
            if let Some(ids) = note_ids.as_array() {
                if ids.is_empty() {
                    println!("No cards found.");
                    return;
                }
                
                // Get note info for found IDs
                let info_params = json!({
                    "notes": ids
                });
                
                match call_ankiconnect("notesInfo", info_params) {
                    Ok(notes) => {
                        let html_output = format_anki_results(notes, &show_fields);
                        println!("{}", html_output);
                    },
                    Err(e) => {
                        eprintln!("Error getting note information: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error searching Anki: {}", e);
        }
    }
}
