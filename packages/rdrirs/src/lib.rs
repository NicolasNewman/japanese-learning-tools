/*
 * Rikaitan Deinflector Reference Implementation
 *
 * Copyright (C) 2023-2025 Ajatt-Tools and contributors
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, visit the http://fsf.org website.
 */

mod rule_sets;

use rule_sets::RULESETS;

/// Rule type flags for matching verb and adjective types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleType(u32);

impl RuleType {
    pub const NONE: RuleType = RuleType(0);
    pub const V1: RuleType = RuleType(1); // Verb ichidan
    pub const V5: RuleType = RuleType(2); // Verb godan
    pub const VS: RuleType = RuleType(4); // Verb suru
    pub const VK: RuleType = RuleType(8); // Verb kuru
    pub const VZ: RuleType = RuleType(16); // Verb zuru
    pub const ADJ_I: RuleType = RuleType(32); // Adjective i
    pub const IRU: RuleType = RuleType(64); // Intermediate -iru endings

    #[inline]
    pub const fn new(value: u32) -> Self {
        RuleType(value)
    }

    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
}

// Bitwise operations for RuleType
impl std::ops::BitOr for RuleType {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        RuleType(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for RuleType {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        RuleType(self.0 & rhs.0)
    }
}

/// A transformation rule for deinflection
#[derive(Debug, Clone, Copy)]
pub struct Reason {
    pub kana_in: &'static str,
    pub kana_out: &'static str,
    pub rules_in: RuleType,
    pub rules_out: RuleType,
}

impl Reason {
    #[inline]
    pub const fn empty(&self) -> bool {
        self.rules_in.bits() == 0
            && self.rules_out.bits() == 0
            && self.kana_in.is_empty()
            && self.kana_out.is_empty()
    }
}

/// Maximum number of rules per ruleset
pub const ARR_MAX_SIZE: usize = 50;

/// A set of related deinflection rules
#[derive(Debug, Clone, Copy)]
pub struct RuleSet {
    pub name: &'static str,
    pub rules: [Reason; ARR_MAX_SIZE],
}

/// A deinflected form with its transformation history
#[derive(Debug, Clone)]
pub struct Deinflection {
    pub term: String,
    pub rules: RuleType,
    pub reasons: Vec<String>,
}

/// Helper to prepend a value to a vector
fn copy_and_prepend<T: Clone>(first: &[T], value: T) -> Vec<T> {
    let mut joined = vec![value];
    joined.extend_from_slice(first);
    joined
}

/// Check if a rule can be applied to a result
fn rule_applies(result: &Deinflection, rule: &Reason) -> bool {
    if result.rules != RuleType::NONE && (rule.rules_in & result.rules) == RuleType::NONE {
        return false;
    }
    if !result.term.ends_with(rule.kana_in) {
        return false;
    }
    if result.term.len() + rule.kana_out.len() <= rule.kana_in.len() {
        return false;
    }
    true
}

/// Deinflect a Japanese word to find possible dictionary forms
pub fn deinflect(source: &str) -> Vec<Deinflection> {
    let mut results = vec![Deinflection {
        term: source.to_string(),
        rules: RuleType::NONE,
        reasons: Vec::new(),
    }];

    let mut idx = 0;
    while idx < results.len() {
        for ruleset in RULESETS.iter() {
            for rule in &ruleset.rules {
                if rule.empty() {
                    // Reached the end of ruleset
                    break;
                }
                if let Some(result) = results.get(idx) {
                    if rule_applies(result, rule) {
                        let term_len = result.term.len();
                        let kana_in_len = rule.kana_in.len();
                        let mut new_term = result.term[..term_len - kana_in_len].to_string();
                        new_term.push_str(rule.kana_out);

                        results.push(Deinflection {
                            term: new_term,
                            rules: rule.rules_out,
                            reasons: copy_and_prepend(&result.reasons, ruleset.name.to_string()),
                        });
                    }
                }
            }
        }
        idx += 1;
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_deinflection() {
        let results = deinflect("食べる");
        assert!(!results.is_empty());
        assert_eq!(results[0].term, "食べる");
    }
}
