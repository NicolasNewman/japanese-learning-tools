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

use std::collections::HashMap;
use thiserror::Error;

/// Custom error type for gd-tools operations
#[derive(Error, Debug)]
pub enum GdToolsError {
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Missing required argument: {0}")]
    MissingArgument(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

pub type Result<T> = std::result::Result<T, GdToolsError>;

/// Parse command line arguments into a HashMap
pub fn parse_args(args: &[String]) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];

        if arg.starts_with("--") {
            let key = arg.trim_start_matches("--").to_string();

            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                result.insert(key, args[i + 1].clone());
                i += 2;
            } else {
                result.insert(key, String::new());
                i += 1;
            }
        } else {
            // Non-flag argument
            result.insert(format!("arg{}", i), arg.clone());
            i += 1;
        }
    }

    result
}

/// Helper function to escape HTML content
pub fn escape_html(content: &str) -> String {
    html_escape::encode_text(content).to_string()
}

/// Generate HTML wrapper for output
pub fn html_wrapper(content: &str, css: Option<&str>) -> String {
    let css_content = css.unwrap_or("");
    format!(
        r#"
        <style>
        {css}
        body {{
            font-family: sans-serif;
            margin: 0;
            padding: 10px;
        }}
        </style>
        {content}"#,
        css = css_content,
        content = content
    )
}
