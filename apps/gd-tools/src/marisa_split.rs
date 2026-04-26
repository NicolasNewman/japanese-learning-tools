use crate::kana_conv::{
    enum_unicode_chars, half_to_full_width, hiragana_to_katakana, katakana_to_hiragana, JpSet,
    KanaInsensitiveMore, Utf8CharView,
};
use crate::util::{parse_args, GdToolsError, Result};
use rdrirs::{deinflect, Deinflection};
use rsmarisa::{Agent, Trie};
use std::path::{Path, PathBuf};
use std::{env, fs};

const CSS_STYLE: &str = r#"
<style>
  .gd-marisa {
    font-size: 2rem;
    margin-bottom: 0.05em;
    margin-top: -0.2em;
    color: #1268c3;
    font-weight: normal;
  }
  .gd-marisa a {
    display: inline-block;
    font-weight: normal;
    color: royalblue;
    text-decoration: none;
    border-bottom: dashed max(1px, calc(1em / 16)) currentColor;
  }
  .gd-marisa a.gd-headword {
    background-color: #ddeeff;
    border-radius: 0.2rem;
    font-weight: 500;
  }
  .gd-marisa > ul {
    --size: 1rem;
    font-size: var(--size);
    padding-inline-start: var(--size);
    margin-block: 2px;
  }
  .gd-marisa .alternatives {
    --size: 1rem;
    display: grid;
    font-size: var(--size);
    gap: calc( var(--size) / 4);
    max-width: 100%;
    margin: 0 auto;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    align-content: start;
    justify-content: space-around;
    text-align: left;
    padding: 5px 0px;
  }
  .gd-marisa .alternatives > ul {
    list-style-type: none;
    margin: 0;
    padding: calc( var(--size) / 4);
    background-color: hsl(150deg 30% 60% / 10%);
  }
</style>
"#;

const HELP_TEXT: &str = r#"usage: gd-tools marisa [OPTIONS]

Split sentence using MARISA and print links to each word.

OPTIONS
  --word WORD          required word
  --sentence SENTENCE  required sentence
  --user-dict PATH     optional path to user dictionary (default: looks for "marisa_words.dic" in resources and user directories)
"#;

const MAX_FORWARD_SEARCH_LEN_BYTES: usize = 3 * 20;

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

    find_file_recursive(&mut possible_dirs, "marisa_words.dic")
}

struct MarisaParams {
    gd_word: String,
    gd_sentence: String,
    user_dict: PathBuf,
}

impl MarisaParams {
    fn new() -> Self {
        Self {
            gd_word: String::new(),
            gd_sentence: String::new(),
            user_dict: find_user_dict_file(),
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
                "Unknown argument: {}",
                key
            )));
        }
        Ok(())
    }
}

fn parse_params(args: &[String]) -> Result<MarisaParams> {
    let mut params = MarisaParams::new();

    let parsed_args = parse_args(args);

    for (key, value) in parsed_args {
        params.assign(&key, &value)?;
    }

    if params.gd_word.is_empty() {
        return Err(GdToolsError::MissingArgument("--word".into()));
    }

    if params.gd_sentence.is_empty() {
        return Err(GdToolsError::MissingArgument("--sentence".into()));
    }

    Ok(params)
}

struct Deinflected {
    from: String,
    to: Vec<Deinflection>,
}

fn find_deinflections_starting_with(search_str: &str) -> Vec<Deinflected> {
    // Loop from larger towards shorter substrings
    enum_unicode_chars(search_str)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|ch| {
            let end_idx = ch.idx + ch.ch.len();
            &search_str[0..end_idx]
        })
        .map(|substr| Deinflected {
            from: substr.to_string(),
            to: deinflect(substr),
        })
        .collect()
}

fn find_keywords_starting_with(agent: &mut Agent, trie: &Trie, search_str: &str) -> JpSet {
    let mut results = JpSet::new();

    let variants = [
        search_str.to_string(),
        hiragana_to_katakana(search_str),
        katakana_to_hiragana(search_str),
    ];

    let deinflections = variants
        .iter()
        .map(|s| find_deinflections_starting_with(s))
        .flatten()
        .flat_map(|group| group.to);

    for deinflection in deinflections {
        agent.set_query_str(deinflection.term.as_str());
        while trie.common_prefix_search(agent) {
            let key = agent.key();
            if let Ok(key_str) = std::str::from_utf8(key.as_bytes()) {
                results.insert(KanaInsensitiveMore::new(key_str.to_string()));
            }
        }
    }

    results
}

fn lookup_words(params: MarisaParams) -> Result<()> {
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

    if !params.user_dict.is_file() {
        return Err(GdToolsError::ServiceUnavailable(format!(
            "Dictionary file not found: {}",
            params.user_dict.display()
        )));
    }

    // Trie
    let mut trie = Trie::new();
    let mut agent = Agent::new();
    let mut pos_in_gd_word: isize = 0 as isize;
    trie.load(&params.user_dict.display().to_string())?;

    println!("<div class=\"gd-marisa\">");
    let mut alternatives: Vec<JpSet> = Vec::new();
    alternatives.reserve(20);

    for Utf8CharView { idx, ch } in enum_unicode_chars(&gd_sentence) {
        let end_pos = (idx + MAX_FORWARD_SEARCH_LEN_BYTES).min(gd_sentence.len());
        let search_substr = &gd_sentence[idx..end_pos];

        let headwords = find_keywords_starting_with(&mut agent, &trie, search_substr);

        let bword = if headwords.is_empty() {
            ch.to_string()
        } else {
            // Find the max by length (longest string)
            headwords
                .iter()
                .max_by_key(|hw| hw.value.len())
                .map(|hw| hw.value.clone())
                .unwrap_or_else(|| ch.to_string())
        };

        if params.gd_word == bword {
            pos_in_gd_word = bword.len() as isize;
        } else {
            pos_in_gd_word -= ch.len() as isize;
        }

        let class_name = if pos_in_gd_word > 0 {
            "gd-headword"
        } else {
            "gd-word"
        };

        let encoded_bword =
            url::form_urlencoded::byte_serialize(bword.as_bytes()).collect::<String>();

        print!(
            r#"<a class="{}" href="bword:{}">{}</a>"#,
            class_name, encoded_bword, ch
        );

        alternatives.push(headwords);
    }

    println!("<div class=\"alternatives\">");
    for group in alternatives.iter().filter(|g| !g.is_empty()) {
        println!("<ul>");
        for word in group {
            let class_attr = if word.value == gd_word {
                "gd-headword"
            } else {
                ""
            };
            let encoded =
                url::form_urlencoded::byte_serialize(word.value.as_bytes()).collect::<String>();
            println!(
                r#"<li><a class="{}" href="bword:{}">{}</a></li>"#,
                class_attr, encoded, word.value
            );
        }
        println!("</ul>");
    }

    println!("</div>"); // close div.alternatives

    println!("</div>"); // close div.gd-marisa
    println!(r#"{}"#, CSS_STYLE);

    Ok(())
}

pub fn split_marisa(args: &[String]) {
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
