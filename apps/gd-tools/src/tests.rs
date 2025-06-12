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
mod tests {
    use crate::kana_conv;
    use crate::util;
    
    #[test]
    fn test_hiragana_to_katakana() {
        assert_eq!(kana_conv::hiragana_to_katakana("あいうえお"), "アイウエオ");
        assert_eq!(kana_conv::hiragana_to_katakana("こんにちは"), "コンニチハ");
        assert_eq!(kana_conv::hiragana_to_katakana("漢字とひらがな"), "漢字トヒラガナ");
    }
    
    #[test]
    fn test_katakana_to_hiragana() {
        assert_eq!(kana_conv::katakana_to_hiragana("アイウエオ"), "あいうえお");
        assert_eq!(kana_conv::katakana_to_hiragana("コンニチハ"), "こんにちは");
        assert_eq!(kana_conv::katakana_to_hiragana("漢字トカタカナ"), "漢字とかたかな");
    }
    
    #[test]
    fn test_full_to_half_width() {
        assert_eq!(kana_conv::full_to_half_width("ＡＢＣ１２３"), "ABC123");
        assert_eq!(kana_conv::full_to_half_width("　"), " ");
    }
    
    #[test]
    fn test_half_to_full_width() {
        assert_eq!(kana_conv::half_to_full_width("ABC123"), "ＡＢＣ１２３");
        assert_eq!(kana_conv::half_to_full_width(" "), "　");
    }
    
    #[test]
    fn test_html_wrapper() {
        let content = "<p>Test</p>";
        let css = "body { color: red; }";
        let html = util::html_wrapper(content, Some(css));
        
        assert!(html.contains("<p>Test</p>"));
        assert!(html.contains("body { color: red; }"));
        assert!(html.contains("<!DOCTYPE html>"));
    }
    
    #[test]
    fn test_parse_args() {
        let args = vec![
            "--name".to_string(),
            "value".to_string(),
            "--flag".to_string(),
            "word".to_string()
        ];
        
        let parsed = util::parse_args(&args);
        
        assert_eq!(parsed.get("name"), Some(&"value".to_string()));
        assert_eq!(parsed.get("flag"), Some(&"".to_string()));
        assert_eq!(parsed.get("arg3"), Some(&"word".to_string()));
    }
}
