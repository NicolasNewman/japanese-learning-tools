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
use reqwest::blocking::Client;
use std::time::Duration;

const TIMEOUT_SECONDS: u64 = 5;
const MAX_IMAGES: usize = 15;
const HELP_TEXT: &str = r#"usage: gd-images [OPTIONS]

Search for images on Bing and display them in an HTML grid.

OPTIONS
  --query STRING   The search query (required)
  --count NUMBER   Maximum number of images to display (default: 15)
  --market STRING  Market code for Bing API (default: "ja-JP")
  -h, --help       Print this help screen

EXAMPLES
gd-images --query "猫"
gd-images --query "東京" --count 10 --market "en-US"
"#;

const CSS_STYLE: &str = r#"
.image-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 10px;
  padding: 10px;
}
.image-item {
  overflow: hidden;
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}
.image-item img {
  width: 100%;
  height: auto;
  display: block;
  transition: transform 0.2s;
}
.image-item:hover img {
  transform: scale(1.05);
}
"#;

fn search_bing_images(query: &str, count: usize, market: &str) -> Result<Vec<String>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECONDS))
        .build()?;
    
    let search_url = format!(
        "https://www.bing.com/images/search?q={}&form=HDRSC2&first=1",
        url_encode(query)
    );
    
    let response = client.get(&search_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .header("Accept-Language", market)
        .send()?
        .text()?;
    
    let image_urls = extract_image_urls(&response, count)?;
    Ok(image_urls)
}

fn url_encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

fn extract_image_urls(html: &str, count: usize) -> Result<Vec<String>> {
    // In a real implementation, this would use a proper HTML parser
    // or regex to extract image URLs from Bing's search results.
    // This is a simplified version that would need to be enhanced.
    
    let mut urls = Vec::new();
    let re = regex::Regex::new(r#""murl":"([^"]*)""#).unwrap();
    
    for cap in re.captures_iter(html).take(count) {
        if let Some(url) = cap.get(1) {
            urls.push(url.as_str().to_string());
        }
    }
    
    if urls.is_empty() {
        return Err(GdToolsError::ServiceUnavailable("No images found".into()));
    }
    
    Ok(urls)
}

fn generate_image_html(image_urls: &[String]) -> String {
    let mut html = String::from("<div class=\"image-grid\">");
    
    for url in image_urls {
        html.push_str(&format!(
            r#"<div class="image-item"><a href="{url}" target="_blank"><img src="{url}" alt="Search result"></a></div>"#,
            url = url
        ));
    }
    
    html.push_str("</div>");
    html_wrapper(&html, Some(CSS_STYLE))
}

fn print_help() {
    println!("{}", HELP_TEXT);
}

pub fn search_images(args: &[String]) {
    if args.is_empty() || args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        print_help();
        return;
    }
    
    let parsed_args = parse_args(args);
    
    // Get search query
    let query = match parsed_args.get("query") {
        Some(q) => q,
        None => {
            eprintln!("Error: Missing required argument --query");
            print_help();
            return;
        }
    };
    
    // Get optional parameters
    let count = parsed_args.get("count")
        .and_then(|c| c.parse::<usize>().ok())
        .unwrap_or(MAX_IMAGES);
    
    let market = parsed_args.get("market")
        .cloned()
        .unwrap_or_else(|| String::from("ja-JP"));
    
    match search_bing_images(query, count, &market) {
        Ok(image_urls) => {
            let html = generate_image_html(&image_urls);
            println!("{}", html);
        },
        Err(e) => {
            eprintln!("Error searching images: {}", e);
        }
    }
}
