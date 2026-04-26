/*
 * Debug test to see what deinflect actually returns
 */

use rdrirs::{deinflect};

fn main() {
    println!("Testing '愛しく' (expected: '愛しい' with 'adv'):");
    let results = deinflect("愛しく");
    for (i, result) in results.iter().enumerate() {
        println!(
            "  [{}] term: '{}', rules: {:?}, reasons: {:?}",
            i, result.term, result.rules, result.reasons
        );
    }

    println!("\nTesting '食べれば' (expected: '食べる' with '-ba'):");
    let results = deinflect("食べれば");
    for (i, result) in results.iter().enumerate() {
        println!(
            "  [{}] term: '{}', rules: {:?}, reasons: {:?}",
            i, result.term, result.rules, result.reasons
        );
    }

    println!("\nTesting '買える' (expected: '買う' with 'potential or passive'):");
    let results = deinflect("買える");
    for (i, result) in results.iter().enumerate() {
        println!(
            "  [{}] term: '{}', rules: {:?}, reasons: {:?}",
            i, result.term, result.rules, result.reasons
        );
        if i > 10 {
            println!("  ... (showing first 11 results)");
            break;
        }
        
    }
}
