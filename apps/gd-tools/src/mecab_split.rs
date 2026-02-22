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

use crate::kana_conv::half_to_full_width;
use crate::util::{parse_args, GdToolsError, Result};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

const CSS_STYLE: &str = r#"
<style>
  * {
    margin: 0;
    padding: 0;
  }
  .gd-mecab {
    font-size: 2rem;
    color: #1268c3;
    font-weight: normal;
  }
  .gd-mecab a {
    display: inline-block;
    font-weight: normal;
    color: royalblue;
    text-decoration: none;
  }
  .gd-mecab a:not(:last-of-type)::after {
    content: "";
    display: inline-block;
    background-color: #333;
    margin: 4px;
    width: 3px;
    height: 3px;
    border-radius: 100vmax;
    vertical-align: middle;
    cursor: text;
    user-select: text;
  }
  .gd-mecab a b {
    background-color: #ddeeff;
    border-radius: 0.2rem;
    font-weight: 500;
  }
</style>
"#;

const HELP_TEXT: &str = r#"usage: gd-mecab [OPTIONS]

Echo input back to GoldenDict as HTML with sentence split into parts.

OPTIONS
  --word %GDWORD%        required word
  --sentence %GDSEARCH%  required sentence
  --user-dict PATH       path to the user dictionary.
"#;

fn find_file_recursive<P: AsRef<Path>>(possible_dirs: &mut Vec<PathBuf>, file_name: P) -> PathBuf {
    let file_name = file_name.as_ref();

    for idx in 0..possible_dirs.len() {
        if !possible_dirs[idx].is_dir() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(&possible_dirs[idx]) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.file_name().unwrap_or_default() == file_name {
                    return path;
                } else if path.is_dir() {
                    possible_dirs.push(path);
                }
            }
        }
    }

    PathBuf::new()
}

fn get_resource_dir() -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path.parent().unwrap();

    #[cfg(target_os = "macos")]
    // TODO: proper path for macOS app bundle
    let resource_dir = exe_dir.parent().unwrap().join("Resources");

    #[cfg(not(target_os = "macos"))]
    let resource_dir = exe_dir
        .parent()
        .unwrap()
        .join("lib/jp-learning-tools/resources");

    resource_dir
}

fn find_user_dict_file() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_default();

    let mut possible_dirs = vec![
        get_resource_dir(),
        PathBuf::from("/usr/share/gd-tools"),
        home.join(".local/share/gd-tools"),
        home.join(".local/share/Anki2/addons21"),
    ];

    find_file_recursive(&mut possible_dirs, "user_dic.dic")
}

fn find_dic_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_default();

    let mut possible_dirs =
        vec![
        get_resource_dir().join("mecab-ipadic-neologd"),
        PathBuf::from("/home/dev-env/repos/japanese-learning-tools/apps/gd-tools/res/mecab-ipadic-neologd"), // neologd is preferred if available
        PathBuf::from("/usr/lib/mecab/dic/mecab-ipadic-neologd"), // neologd is preferred if available
        PathBuf::from("/usr/lib/mecab/dic"),
        PathBuf::from("/usr/lib64/mecab/dic"),
        home.join(".local/share/Anki2/addons21"),
    ];

    let path = find_file_recursive(&mut possible_dirs, "dicrc");
    if !path.as_os_str().is_empty() {
        path.parent().unwrap_or(&path).to_path_buf()
    } else {
        PathBuf::new()
    }
}

struct MecabParams {
    gd_word: String,
    gd_sentence: String,
    user_dict: PathBuf,
    dic_dir: PathBuf,
}

impl MecabParams {
    fn new() -> Self {
        Self {
            gd_word: String::new(),
            gd_sentence: String::new(),
            user_dict: find_user_dict_file(),
            dic_dir: find_dic_dir(),
        }
    }

    fn assign(&mut self, key: &str, value: &str) -> Result<()> {
        if key == "word" {
            self.gd_word = value.to_string();
        } else if key == "sentence" {
            self.gd_sentence = value.to_string();
        } else if key == "user-dict" {
            self.user_dict = PathBuf::from(value);
        } else {
            return Err(GdToolsError::InvalidArgument(format!(
                "Unknown argument name: {}",
                key
            )));
        }
        Ok(())
    }
}

fn replace_all(mut str: String, from: &str, to: &str) -> String {
    let mut start_pos = 0;
    while let Some(pos) = str[start_pos..].find(from) {
        let actual_pos = start_pos + pos;
        str.replace_range(actual_pos..actual_pos + from.len(), to);
        start_pos = actual_pos + to.len();
    }
    str
}

fn lookup_words(params: MecabParams) -> Result<()> {
    // Apply full-width conversion to word
    let mut gd_word = half_to_full_width(&params.gd_word);
    // Remove spaces
    gd_word.retain(|c| !c.is_whitespace());

    // Process sentence
    let mut gd_sentence = params.gd_sentence.clone();
    gd_sentence.retain(|c| !c.is_whitespace());

    // If sentence is empty, use the word
    if gd_sentence.is_empty() {
        gd_sentence = gd_word.clone();
    } else {
        gd_sentence = half_to_full_width(&gd_sentence);
    }

    // Verify dictionary directory exists
    if !params.dic_dir.is_dir() {
        return Err(GdToolsError::ServiceUnavailable(
            "Couldn't find dictionary directory.".into(),
        ));
    }

    // Create MeCab arguments
    let dicdir = format!("--dicdir={}", params.dic_dir.display());

    let mut args = vec![
        dicdir,
        r#"--node-format=<a href="bword:%f[6]" title="%f[6]">%m</a>"#.to_string(),
        r#"--unk-format=<a href="bword:%m" title="%m">%m</a>"#.to_string(),
        "--eos-format=<br>".to_string(),
    ];

    if params.user_dict.is_file() {
        args.push(format!("--userdic={}", params.user_dict.display()));
    }

    // Execute MeCab with the provided arguments
    let mut cmd = Command::new("mecab");
    for arg in &args {
        cmd.arg(arg);
    }

    // Create a process to run MeCab
    let mut child = cmd
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| {
            GdToolsError::ServiceUnavailable(format!("Failed to spawn MeCab process: {}", e))
        })?;

    // Write the input text to MeCab's stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(gd_sentence.as_bytes()).map_err(|e| {
            GdToolsError::ServiceUnavailable(format!("Failed to write to MeCab process: {}", e))
        })?;
        // Closing stdin is important to avoid deadlocks
    }

    // Get output from MeCab
    let output = child.wait_with_output().map_err(|e| {
        GdToolsError::ServiceUnavailable(format!("Failed to get MeCab output: {}", e))
    })?;

    if !output.status.success() {
        return Err(GdToolsError::ServiceUnavailable(
            "MeCab process failed".into(),
        ));
    }

    // Convert output to string
    let result = String::from_utf8(output.stdout).map_err(|e| {
        GdToolsError::ServiceUnavailable(format!("Invalid UTF-8 output from MeCab: {}", e))
    })?;

    if result.is_empty() {
        return Err(GdToolsError::ServiceUnavailable(
            "Failed to parse text with MeCab.".into(),
        ));
    }

    // Highlight word
    let highlighted = replace_all(
        result,
        &format!(">{}<", gd_word),
        &format!("><b>{}</b><", gd_word),
    );

    println!(r#"<div class="gd-mecab">{}</div>"#, highlighted);
    println!("{}", CSS_STYLE);

    // Debug info (not shown in GD)
    println!(r#"<div style="display: none;">"#);
    println!("dicdir: {}", params.dic_dir.display());
    println!("userdic: {}", params.user_dict.display());

    println!("mecab args: [{}]", args.join(", "));
    println!(r#"</div>"#);

    Ok(())
}

fn parse_params(args: &[String]) -> Result<MecabParams> {
    let mut params = MecabParams::new();

    let parsed_args = parse_args(args);

    for (key, value) in parsed_args {
        params.assign(&key, &value)?;
    }

    if params.gd_word.is_empty() {
        return Err(GdToolsError::MissingArgument("--word".into()));
    }

    Ok(params)
}

pub fn split_mecab(args: &[String]) {
    if args.is_empty()
        || args.contains(&String::from("--help"))
        || args.contains(&String::from("-h"))
    {
        println!("{}", HELP_TEXT);
        return;
    }

    match parse_params(args) {
        Ok(params) => {
            if let Err(e) = lookup_words(params) {
                eprintln!("{}", e);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            println!("{}", HELP_TEXT);
        }
    }
}
