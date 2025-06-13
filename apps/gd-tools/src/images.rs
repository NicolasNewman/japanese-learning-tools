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

use crate::util::{GdToolsError, Result, parse_args};
use regex::Regex;
use reqwest::blocking::Client;
use std::time::Duration;

const MAX_TIME_SECONDS: u64 = 6;
const MAX_IMAGES: usize = 5;
const HELP_TEXT: &str = r#"usage: gd-images [OPTIONS]

Get images from Bing.

OPTIONS
  --max-time SECONDS  maximum time in seconds to wait for response.
  --word WORD         search term.

EXAMPLES
  gd-images --max-time 6 --word "犬"
  gd-images --word 猫
"#;

const CSS_STYLE: &str = r#"<style>
    .gallery {
        display: grid;
        gap: 10px;
        margin: 0;
        justify-items: center;
        align-items: start;
        align-content: start;
        justify-content: space-between;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    }
    .gallery img {
        margin: 0 auto;
        max-width: 100%;
        width: 100%;
        border-radius: 5px;
        display: block;
        max-height: 95vh;
        object-fit: contain;
    }
</style>"#;

struct ImagesParams {
    max_time: u64,
    word: String,
}

fn fetch_images(params: &ImagesParams) -> Result<()> {
    let client = Client::builder()
        .timeout(Duration::from_secs(params.max_time))
        .danger_accept_invalid_certs(true)
        .build()?;
    
    let response = client.get("https://www.bing.com/images/search")
        .query(&[("q", &params.word), ("mkt", &String::from("ja-JP"))])
        .header("User-Agent", "Mozilla/5.0")
        .send()?
        .text()?;
    
    let img_re = Regex::new(r#"<img[^<>]*class="mimg[^<>]*>"#).map_err(|_| {
        GdToolsError::ServiceUnavailable("Failed to compile regex".into())
    })?;
    
    println!("<div class=\"gallery\">");
    
    let mut count = 0;
    for cap in img_re.find_iter(&response) {
        if count >= MAX_IMAGES {
            break;
        }
        println!("{}", cap.as_str());
        count += 1;
    }
    
    println!("</div>");
    println!("{}", CSS_STYLE);
    
    Ok(())
}

fn parse_images_params(args: &[String]) -> Result<ImagesParams> {
    let mut params = ImagesParams {
        max_time: MAX_TIME_SECONDS,
        word: String::new(),
    };
    
    let parsed_args = parse_args(args);
    
    // Parse max_time parameter
    if let Some(max_time_str) = parsed_args.get("max-time") {
        if let Ok(max_time) = max_time_str.parse::<u64>() {
            params.max_time = max_time;
        }
    }
    
    // Parse word parameter
    if let Some(word) = parsed_args.get("word") {
        params.word = word.clone();
    } else {
        // Word is required
        return Err(GdToolsError::MissingArgument("--word".into()));
    }
    
    Ok(params)
}

pub fn images(args: &[String]) {
    if args.is_empty() || args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        println!("{}", HELP_TEXT);
        return;
    }
    
    match parse_images_params(args) {
        Ok(params) => {
            if let Err(err) = fetch_images(&params) {
                eprintln!("{}", err);
            }
        },
        Err(err) => {
            eprintln!("{}", err);
            println!("{}", HELP_TEXT);
        }
    }
}
