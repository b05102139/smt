use std::collections::HashMap;
//use regex::Regex;

fn get_stats(vocab: HashMap<&str, i32>) -> HashMap<(&str, &str), i32> {
    let mut pairs: HashMap<(&str, &str), i32> = HashMap::new();
    for (key, freq) in &vocab {
        let mut symbols: Vec<&str> = key.split_whitespace().collect();
        for i in 0..(symbols.len()-1) {
            *pairs.entry((symbols[i], symbols[i+1])).or_insert(0) += freq
        }
    }
    return pairs
}

fn merge_vocab(pair: (&str, &str), v_in: HashMap<&str, i32>) -> HashMap<&str, i32> {
    let mut v_out: HashMap<(&str, i32> = HashMap::new();
    v_out
}

// pub the final bpe algo, rest is internal
// let vocab_1 = HashMap::from([("a b", 1), ("a d a b", 1)]);