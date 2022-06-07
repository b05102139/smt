use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::collections::HashMap;
use std::string::String;

pub struct Reader {
    e_corpus_path: &'static str,
    f_corpus_path: &'static str,
    e_words: HashMap<String, i32>,
    f_words: HashMap<String, i32>,
}

pub impl Reader {
    pub fn new(e_corpus_path: &'static str, f_corpus_path: &'static str) -> Self {
        Self {
            e_corpus_path,
            f_corpus_path,
            e_words: fill_dict(e_corpus_path, false).unwrap(),
            f_words: fill_dict(f_corpus_path, true).unwrap(),
        }
    }
}

fn fill_dict(corpus_path: &'static str, add_null_to_src: bool) -> std::io::Result<HashMap<String, i32>> {
    let mut lexicon: HashMap<String, i32> = HashMap::new();
    let input = File::open(corpus_path)?;
    let mut index = 0;

    if add_null_to_src == true {
        lexicon.insert(String::from("NULL"), 0);
        index += 1;
    }

    let buffered_reader = BufReader::new(input);
    for line in buffered_reader.lines() {
        let line = line?.to_lowercase(); // should be boolean; change later
        let words = line.split_whitespace();
        for word in words {
            if !lexicon.contains_key(word) {
                lexicon.insert(String::from(word), index);
                index += 1;
            }
        }
    }
    Ok(lexicon)
}


/*

let origin = "時間 の あ る 時 に 私 に 会 い に 来 なさ い 。";
let mut vec1: Vec<String> = vec![];
for i in origin.split_whitespace() {
    let word_map = myMod.prob_table[i].clone();
    let word2 = &word_map.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).map(|(k, _v)| k).unwrap();
    //print!("{:?}", word);
    vec1.push(String::from(word2.clone()));
}
//let joined = vec1.join("");
println!("{:?}", origin)
println!("{:?}", vec1)
//println!("{:?}", joined)

let mut ex1 = Reader::new("data/fbis_small.en", "data/fbis_small.zh");
let ex2 = iterate(&ex1, 5).unwrap();
let mut vec: Vec<f64> = Vec::new();

let ex1 = Reader::new("data/train.ja.000", "data/train.en.000");
let mut myMod = IbmModel1::new(ex1);
myMod.e_words_len
myMod.iterate(30);
myMod.total_iterations
myMod.prob_table["Yili"]

*/