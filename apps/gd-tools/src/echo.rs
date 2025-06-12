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

use crate::util::{GdToolsError, Result, parse_args, html_wrapper};
use std::path::PathBuf;
use std::process::Command;
use std::fs;

const STROKE_ORDER_HELP: &str = r#"usage: gd-strokeorder [OPTIONS]

Show stroke order of characters by rendering them with the KanjiStrokeOrders font.

OPTIONS
  --text TEXT    The text to display (required)
  --size NUMBER  Font size (default: 60)
  -h, --help     Print this help screen

EXAMPLES
gd-strokeorder --text "漢字" 
gd-strokeorder --text "日本語" --size 80
"#;

const HANDWRITTEN_HELP: &str = r#"usage: gd-handwritten [OPTIONS]

Show handwritten form of characters by rendering them with the ArmedLemon font.

OPTIONS
  --text TEXT    The text to display (required)
  --size NUMBER  Font size (default: 60)
  -h, --help     Print this help screen

EXAMPLES
gd-handwritten --text "漢字" 
gd-handwritten --text "日本語" --size 80
"#;

const CSS_STYLE: &str = r#"
.character-display {
  text-align: center;
  padding: 20px;
}
.character {
  display: inline-block;
  margin: 0 10px;
  vertical-align: middle;
}
.character img {
  max-width: 100%;
  height: auto;
}
"#;

fn get_font_path(font_name: &str) -> Result<PathBuf> {
    // This is a simplified implementation. In reality, you'd need
    // to check multiple locations where fonts might be installed.
    let font_dir = match std::env::var("XDG_DATA_HOME") {
        Ok(dir) => PathBuf::from(dir).join("fonts"),
        Err(_) => {
            let home = dirs::home_dir().ok_or_else(|| 
                GdToolsError::ServiceUnavailable("Could not find home directory".into())
            )?;
            home.join(".local/share/fonts")
        }
    };
    
    // Check for different font file extensions
    for ext in &["ttf", "TTF", "otf", "OTF"] {
        let path = font_dir.join(format!("{}.{}", font_name, ext));
        if path.exists() {
            return Ok(path);
        }
    }
    
    // Check in app resource directory
    let exec_path = std::env::current_exe()?;
    let app_dir = exec_path.parent().ok_or_else(|| 
        GdToolsError::ServiceUnavailable("Could not determine application directory".into())
    )?;
    
    let res_dir = app_dir.join("res");
    for ext in &["ttf", "TTF", "otf", "OTF"] {
        let path = res_dir.join(format!("{}.{}", font_name, ext));
        if path.exists() {
            return Ok(path);
        }
    }
    
    Err(GdToolsError::ServiceUnavailable(format!("Font {} not found", font_name)))
}

fn generate_font_image(text: &str, font_path: &PathBuf, size: u32) -> Result<String> {
    // Create a temporary directory to store the image
    let temp_dir = std::env::temp_dir().join("gd-tools");
    fs::create_dir_all(&temp_dir)?;
    
    let output_file = temp_dir.join(format!("font_image_{}.png", chrono::Utc::now().timestamp()));
    
    // Use ImageMagick to render the text with the specified font
    let status = Command::new("convert")
        .args([
            "-background", "transparent",
            "-fill", "black",
            "-font", &font_path.to_string_lossy(),
            "-pointsize", &size.to_string(),
            &format!("label:{}", text),
            &output_file.to_string_lossy()
        ])
        .status()?;
    
    if !status.success() {
        return Err(GdToolsError::ServiceUnavailable(
            "Failed to generate image with ImageMagick".into()
        ));
    }
    
    // Convert the image to a data URI
    let image_data = fs::read(&output_file)?;
    let base64_data = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &image_data);
    let data_uri = format!("data:image/png;base64,{}", base64_data);
    
    // Clean up
    let _ = fs::remove_file(output_file);
    
    Ok(data_uri)
}

fn generate_html_for_characters(characters: &str, image_data: &str) -> String {
    let html = format!(
        r#"<div class="character-display">
            <div class="character">
                <img src="{}" alt="{}" />
            </div>
        </div>"#,
        image_data, characters
    );
    
    html_wrapper(&html, Some(CSS_STYLE))
}

fn print_stroke_order_help() {
    println!("{}", STROKE_ORDER_HELP);
}

fn print_handwritten_help() {
    println!("{}", HANDWRITTEN_HELP);
}

pub fn show_stroke_order(args: &[String]) {
    if args.is_empty() || args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        print_stroke_order_help();
        return;
    }
    
    let parsed_args = parse_args(args);
    
    // Get text to display
    let text = match parsed_args.get("text") {
        Some(t) => t,
        None => {
            eprintln!("Error: Missing required argument --text");
            print_stroke_order_help();
            return;
        }
    };
    
    // Get font size
    let size = parsed_args.get("size")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(60);
    
    // Get font path
    match get_font_path("KanjiStrokeOrders") {
        Ok(font_path) => {
            match generate_font_image(text, &font_path, size) {
                Ok(image_data) => {
                    let html = generate_html_for_characters(text, &image_data);
                    println!("{}", html);
                },
                Err(e) => {
                    eprintln!("Error generating stroke order image: {}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

pub fn show_handwritten(args: &[String]) {
    if args.is_empty() || args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        print_handwritten_help();
        return;
    }
    
    let parsed_args = parse_args(args);
    
    // Get text to display
    let text = match parsed_args.get("text") {
        Some(t) => t,
        None => {
            eprintln!("Error: Missing required argument --text");
            print_handwritten_help();
            return;
        }
    };
    
    // Get font size
    let size = parsed_args.get("size")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(60);
    
    // Get font path
    match get_font_path("ArmedLemon") {
        Ok(font_path) => {
            match generate_font_image(text, &font_path, size) {
                Ok(image_data) => {
                    let html = generate_html_for_characters(text, &image_data);
                    println!("{}", html);
                },
                Err(e) => {
                    eprintln!("Error generating handwritten image: {}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
