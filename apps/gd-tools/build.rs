use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Create a resources directory in the output folder
    let res_dir = Path::new(&out_dir).join("res");
    fs::create_dir_all(&res_dir).unwrap();
    
    // Get the source resources directory
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let source_res_dir = Path::new(&manifest_dir).join("gd-tools").join("res");
        
        // If the resources directory exists, copy necessary files
        if source_res_dir.exists() {
            // Copy font files if they exist
            let fonts = ["KanjiStrokeOrders_v4.004.ttf", "ArmedLemon.ttf"];
            
            for font in &fonts {
                let src = source_res_dir.join(font);
                if src.exists() {
                    let dst = res_dir.join(font);
                    fs::copy(&src, &dst).unwrap_or_else(|e| {
                        eprintln!("Warning: Could not copy font {}: {}", font, e);
                        0
                    });
                } else {
                    println!("cargo:warning=Font file {} not found", font);
                }
            }
            
            // Copy dictionary files if they exist
            let dict_files = ["marisa_words.dic", "user_dic.dic"];
            
            for dict in &dict_files {
                let src = source_res_dir.join(dict);
                if src.exists() {
                    let dst = res_dir.join(dict);
                    fs::copy(&src, &dst).unwrap_or_else(|e| {
                        eprintln!("Warning: Could not copy dictionary {}: {}", dict, e);
                        0
                    });
                } else {
                    println!("cargo:warning=Dictionary file {} not found", dict);
                }
            }
        } else {
            println!("cargo:warning=Resources directory not found: {:?}", source_res_dir);
        }
    }
}
