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

/// Convert hiragana to katakana
pub fn hiragana_to_katakana(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    
    for c in text.chars() {
        let code = c as u32;
        // Hiragana range is U+3041 to U+3096
        if code >= 0x3041 && code <= 0x3096 {
            // Convert to katakana by adding the offset between hiragana and katakana
            result.push(char::from_u32(code + 0x60).unwrap_or(c));
        } else {
            result.push(c);
        }
    }
    
    result
}

/// Convert katakana to hiragana
pub fn katakana_to_hiragana(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    
    for c in text.chars() {
        let code = c as u32;
        // Katakana range is U+30A1 to U+30F6
        if code >= 0x30A1 && code <= 0x30F6 {
            // Convert to hiragana by subtracting the offset between katakana and hiragana
            result.push(char::from_u32(code - 0x60).unwrap_or(c));
        } else {
            result.push(c);
        }
    }
    
    result
}

/// Convert full-width characters to half-width
pub fn full_to_half_width(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    
    for c in text.chars() {
        let code = c as u32;
        // Full-width characters are in various ranges
        if code >= 0xFF01 && code <= 0xFF5E {
            // Convert to ASCII
            result.push(char::from_u32(code - 0xFEE0).unwrap_or(c));
        } else if c == '　' {
            // Full-width space
            result.push(' ');
        } else {
            result.push(c);
        }
    }
    
    result
}

/// Convert half-width characters to full-width
pub fn half_to_full_width(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    
    for c in text.chars() {
        let code = c as u32;
        // ASCII printable characters
        if code >= 0x21 && code <= 0x7E {
            // Convert to full-width
            result.push(char::from_u32(code + 0xFEE0).unwrap_or(c));
        } else if c == ' ' {
            // Space to full-width space
            result.push('　');
        } else {
            result.push(c);
        }
    }
    
    result
}

/// Convert romaji to hiragana
pub fn romaji_to_hiragana(text: &str) -> String {
    // This is a simplified implementation
    // In a real implementation, you'd need a more comprehensive mapping table
    // and proper handling of special cases
    
    lazy_static::lazy_static! {
        static ref ROMAJI_TO_HIRAGANA: HashMap<&'static str, &'static str> = {
            let mut m = HashMap::new();
            // Basic hiragana
            m.insert("a", "あ"); m.insert("i", "い"); m.insert("u", "う");
            m.insert("e", "え"); m.insert("o", "お");
            
            // K row
            m.insert("ka", "か"); m.insert("ki", "き"); m.insert("ku", "く");
            m.insert("ke", "け"); m.insert("ko", "こ");
            
            // S row
            m.insert("sa", "さ"); m.insert("shi", "し"); m.insert("su", "す");
            m.insert("se", "せ"); m.insert("so", "そ");
            
            // T row
            m.insert("ta", "た"); m.insert("chi", "ち"); m.insert("tsu", "つ");
            m.insert("te", "て"); m.insert("to", "と");
            
            // N row
            m.insert("na", "な"); m.insert("ni", "に"); m.insert("nu", "ぬ");
            m.insert("ne", "ね"); m.insert("no", "の");
            
            // H row
            m.insert("ha", "は"); m.insert("hi", "ひ"); m.insert("fu", "ふ");
            m.insert("he", "へ"); m.insert("ho", "ほ");
            
            // M row
            m.insert("ma", "ま"); m.insert("mi", "み"); m.insert("mu", "む");
            m.insert("me", "め"); m.insert("mo", "も");
            
            // Y row
            m.insert("ya", "や"); m.insert("yu", "ゆ"); m.insert("yo", "よ");
            
            // R row
            m.insert("ra", "ら"); m.insert("ri", "り"); m.insert("ru", "る");
            m.insert("re", "れ"); m.insert("ro", "ろ");
            
            // W row
            m.insert("wa", "わ"); m.insert("wo", "を");
            
            // N
            m.insert("n", "ん");
            
            // Add more mappings as needed
            
            m
        };
    }
    
    // Very simplified conversion (a real implementation would be more complex)
    // This doesn't handle combinatorics like きゃ (kya)
    let lower = text.to_lowercase();
    let mut result = String::new();
    let mut i = 0;
    
    while i < lower.len() {
        let mut found = false;
        
        // Try to match longer sequences first
        for len in (1..=3).rev() {
            if i + len <= lower.len() {
                let substr = &lower[i..i+len];
                if let Some(&hiragana) = ROMAJI_TO_HIRAGANA.get(&substr) {
                    result.push_str(hiragana);
                    i += len;
                    found = true;
                    break;
                }
            }
        }
        
        if !found {
            // If no match found, keep the original character
            result.push(text.chars().nth(i).unwrap());
            i += 1;
        }
    }
    
    result
}