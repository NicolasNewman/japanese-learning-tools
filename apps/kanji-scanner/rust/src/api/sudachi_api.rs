use std::fs;
use std::sync::{Arc, OnceLock};
use sudachi::analysis::stateless_tokenizer::StatelessTokenizer;
use sudachi::analysis::Tokenize;
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::storage::SudachiDicData;
use sudachi::prelude::*;

static TOKENIZER: OnceLock<StatelessTokenizer<Arc<JapaneseDictionary>>> = OnceLock::new();

fn get_tokenizer() -> &'static StatelessTokenizer<Arc<JapaneseDictionary>> {
    TOKENIZER.get_or_init(|| {
        panic!("Tokenizer not initialized. Call sudachi_init_with_dict_path first!");
    })
}

pub struct TokenInfo {
    pub surface: String,
    pub normalized: String,
    pub reading: String,
    pub part_of_speech: String,
}

#[flutter_rust_bridge::frb]
pub fn sudachi_rs(text: String) -> Vec<TokenInfo> {
    let tokenizer = get_tokenizer();

    let morphemes = tokenizer
        .tokenize(&text, Mode::C, false)
        .expect("Tokenization failed");

    let mut results = Vec::new();
    for i in 0..morphemes.len() {
        let m = morphemes.get(i);
        results.push(TokenInfo {
            surface: m.surface().to_string(),
            normalized: m.normalized_form().to_string(),
            reading: m.reading_form().to_string(),
            part_of_speech: m.part_of_speech().join(","),
        });
    }

    results
}

#[flutter_rust_bridge::frb(sync)]
pub fn sudachi_init_with_dict_path(dict_path: String) -> Result<(), String> {
    TOKENIZER.get_or_init(|| {
        // Use embedded config which includes char.def, unk.def, and rewrite.def
        let config = Config::new_embedded().expect("Failed to load embedded config");

        // Load the system dictionary from the provided path
        let dict_bytes = fs::read(&dict_path)
            .map_err(|e| format!("Failed to read dictionary file at {}: {}", dict_path, e))
            .expect("Failed to read dictionary");

        let storage = SudachiDicData::new(sudachi::dic::storage::Storage::Owned(dict_bytes));

        // Use the embedded chardef version which doesn't require char.def file
        let dict = JapaneseDictionary::from_cfg_storage_with_embedded_chardef(&config, storage)
            .expect("Failed to create dictionary");

        StatelessTokenizer::new(Arc::new(dict))
    });
    Ok(())
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
