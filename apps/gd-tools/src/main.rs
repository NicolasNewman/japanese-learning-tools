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

#[cfg(test)]
mod tests;

mod anki_search;
mod echo;
mod images;
mod kana_conv;
mod marian_mt;
mod marisa_split;
mod mecab_split;
mod translate;
mod util;

use std::env;

const HELP_TEXT: &str = r#"usage: gd-tools ACTION [OPTIONS]
A set of helpful programs to enhance GoldenDict for immersion learning.

ACTIONS
  ankisearch  Search word in Anki.
  images      Get images from Bing.
  translate   Translate text using argostranslate.
  mecab       Split search string using Mecab.
  marisa      Split search string using MARISA.
  strokeorder Show stroke order of a word.
  handwritten Display the handwritten form of a word.

OPTIONS
  -h,--help  Print this help screen.

EXAMPLES
gd-tools ankisearch --field-name VocabKanji %GDWORD%
gd-tools ankisearch --field-name VocabKanji %GDWORD%
gd-tools ankisearch --deck-name Mining %GDWORD%
gd-tools marisa --word %GDWORD% --sentence %GDSENTENCE%
"#;

fn print_help() {
    let program_name = env::args()
        .next()
        .unwrap_or_else(|| String::from("gd-tools"));
    println!("{}", HELP_TEXT.replace("{}", &program_name));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 || args[1] == "-h" || args[1] == "--help" {
        print_help();
        return;
    }

    match args[1].as_str() {
        "ankisearch" => {
            let cmd_args = args[2..].to_vec();
            anki_search::search_anki_cards(&cmd_args);
        }
        "images" => {
            let cmd_args = args[2..].to_vec();
            images::images(&cmd_args);
        }
        "translate" => {
            let cmd_args = args[2..].to_vec();
            translate::translate_text(&cmd_args);
        }
        "mecab" => {
            let cmd_args = args[2..].to_vec();
            mecab_split::split_mecab(&cmd_args);
        }
        "marisa" => {
            let cmd_args = args[2..].to_vec();
            marisa_split::split_marisa(&cmd_args);
        }
        "strokeorder" => {
            let cmd_args = args[2..].to_vec();
            echo::show_stroke_order(&cmd_args);
        }
        "handwritten" => {
            let cmd_args = args[2..].to_vec();
            echo::show_handwritten(&cmd_args);
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            print_help();
        }
    }
}
