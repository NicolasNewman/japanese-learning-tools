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

use rdrirs::{deinflect, Deinflection, RuleType};

#[derive(Debug)]
struct TestCase {
    term: &'static str,
    source: &'static str,
    rule: RuleType,
    reasons: &'static [&'static str],
}

fn find_matching_deinflection<'a>(
    results: &'a [Deinflection],
    term: &str,
    rule: RuleType,
    reasons: &[&str],
) -> Option<&'a Deinflection> {
    results.iter().find(|d| {
        d.term == term
            && (d.rules == RuleType::NONE || (rule & d.rules) == rule)
            && d.reasons.len() == reasons.len()
            && d.reasons.iter().zip(reasons.iter()).all(|(a, b)| a == b)
    })
}

#[test]
fn test_adjective_deinflections() {
    let test_cases = vec![
        TestCase {
            term: "愛しい",
            source: "愛しい",
            rule: RuleType::ADJ_I,
            reasons: &[],
        },
        TestCase {
            term: "愛しい",
            source: "愛しそう",
            rule: RuleType::ADJ_I,
            reasons: &["-sou"],
        },
        TestCase {
            term: "愛しい",
            source: "愛しすぎる",
            rule: RuleType::ADJ_I,
            reasons: &["-sugiru"],
        },
        TestCase {
            term: "愛しい",
            source: "愛しかったら",
            rule: RuleType::ADJ_I,
            reasons: &["-tara"],
        },
        TestCase {
            term: "愛しい",
            source: "愛しかったり",
            rule: RuleType::ADJ_I,
            reasons: &["-tari"],
        },
        TestCase {
            term: "愛しい",
            source: "愛しくて",
            rule: RuleType::ADJ_I,
            reasons: &["-te"],
        },
        TestCase {
            term: "愛しい",
            source: "愛しく",
            rule: RuleType::ADJ_I,
            reasons: &["adv"],
        },
        TestCase {
            term: "愛しい",
            source: "愛しくない",
            rule: RuleType::ADJ_I,
            reasons: &["negative"],
        },
        TestCase {
            term: "愛しい",
            source: "愛しさ",
            rule: RuleType::ADJ_I,
            reasons: &["noun"],
        },
        TestCase {
            term: "愛しい",
            source: "愛しかった",
            rule: RuleType::ADJ_I,
            reasons: &["past"],
        },
    ];

    for test in test_cases {
        let results = deinflect(test.source);
        let found = find_matching_deinflection(&results, test.term, test.rule, test.reasons);
        assert!(
            found.is_some(),
            "Failed for source '{}', expected term '{}' with reasons {:?}",
            test.source,
            test.term,
            test.reasons
        );
    }
}

#[test]
fn test_verb_v1_deinflections() {
    let test_cases = vec![
        TestCase {
            term: "食べる",
            source: "食べる",
            rule: RuleType::V1,
            reasons: &[],
        },
        TestCase {
            term: "食べる",
            source: "食べます",
            rule: RuleType::V1,
            reasons: &["polite"],
        },
        TestCase {
            term: "食べる",
            source: "食べた",
            rule: RuleType::V1,
            reasons: &["past"],
        },
        TestCase {
            term: "食べる",
            source: "食べました",
            rule: RuleType::V1,
            reasons: &["polite past"],
        },
        TestCase {
            term: "食べる",
            source: "食べて",
            rule: RuleType::V1,
            reasons: &["-te"],
        },
        TestCase {
            term: "食べる",
            source: "食べられる",
            rule: RuleType::V1,
            reasons: &["potential or passive"],
        },
        TestCase {
            term: "食べる",
            source: "食べさせる",
            rule: RuleType::V1,
            reasons: &["causative"],
        },
        TestCase {
            term: "食べる",
            source: "食べろ",
            rule: RuleType::V1,
            reasons: &["imperative"],
        },
        TestCase {
            term: "食べる",
            source: "食べない",
            rule: RuleType::V1,
            reasons: &["negative"],
        },
        TestCase {
            term: "食べる",
            source: "食べません",
            rule: RuleType::V1,
            reasons: &["polite negative"],
        },
        TestCase {
            term: "食べる",
            source: "食べれば",
            rule: RuleType::V1,
            reasons: &["-ba"],
        },
        TestCase {
            term: "食べる",
            source: "食べちゃう",
            rule: RuleType::V1,
            reasons: &["-chau"],
        },
        TestCase {
            term: "食べる",
            source: "食べなさい",
            rule: RuleType::V1,
            reasons: &["-nasai"],
        },
        TestCase {
            term: "食べる",
            source: "食べたい",
            rule: RuleType::V1,
            reasons: &["-tai"],
        },
        TestCase {
            term: "食べる",
            source: "食べたら",
            rule: RuleType::V1,
            reasons: &["-tara"],
        },
        TestCase {
            term: "食べる",
            source: "食べよう",
            rule: RuleType::V1,
            reasons: &["volitional"],
        },
    ];

    for test in test_cases {
        let results = deinflect(test.source);
        let found = find_matching_deinflection(&results, test.term, test.rule, test.reasons);
        assert!(
            found.is_some(),
            "Failed for source '{}', expected term '{}' with reasons {:?}",
            test.source,
            test.term,
            test.reasons
        );
    }
}

#[test]
fn test_verb_v5_deinflections() {
    let test_cases = vec![
        TestCase {
            term: "買う",
            source: "買う",
            rule: RuleType::V5,
            reasons: &[],
        },
        TestCase {
            term: "買う",
            source: "買います",
            rule: RuleType::V5,
            reasons: &["polite"],
        },
        TestCase {
            term: "買う",
            source: "買った",
            rule: RuleType::V5,
            reasons: &["past"],
        },
        TestCase {
            term: "買う",
            source: "買って",
            rule: RuleType::V5,
            reasons: &["-te"],
        },
        TestCase {
            term: "買う",
            source: "買える",
            rule: RuleType::V5,
            reasons: &["potential"],
        },
        TestCase {
            term: "買う",
            source: "買わせる",
            rule: RuleType::V5,
            reasons: &["causative"],
        },
        TestCase {
            term: "買う",
            source: "買え",
            rule: RuleType::V5,
            reasons: &["imperative"],
        },
        TestCase {
            term: "買う",
            source: "買わない",
            rule: RuleType::V5,
            reasons: &["negative"],
        },
    ];

    for test in test_cases {
        let results = deinflect(test.source);
        let found = find_matching_deinflection(&results, test.term, test.rule, test.reasons);
        assert!(
            found.is_some(),
            "Failed for source '{}', expected term '{}' with reasons {:?}",
            test.source,
            test.term,
            test.reasons
        );
    }
}

#[test]
fn test_multiple_deinflections() {
    // Test cases with multiple transformations
    let results = deinflect("食べなかった");
    let found = find_matching_deinflection(&results, "食べる", RuleType::V1, &["negative", "past"]);
    assert!(
        found.is_some(),
        "Failed to find '食べる' from '食べなかった' with reasons ['negative', 'past']"
    );

    let results = deinflect("食べられない");
    let found = find_matching_deinflection(
        &results,
        "食べる",
        RuleType::V1,
        &["potential or passive", "negative"],
    );
    assert!(
        found.is_some(),
        "Failed to find '食べる' from '食べられない' with reasons ['potential or passive', 'negative']"
    );
}

#[test]
fn test_no_false_positives() {
    // Test that non-conjugated words don't produce spurious results
    let results = deinflect("猫");
    // Should only have the original term
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].term, "猫");
    assert_eq!(results[0].reasons.len(), 0);
}
