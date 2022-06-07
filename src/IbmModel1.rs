use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::string::String;

pub struct IbmModel1 {
    pub e_corpus_path: String,
    pub f_corpus_path: String,
    pub e_words: HashMap<String, i32>,
    pub f_words: HashMap<String, i32>,
    e_words_inverted: HashMap<i32, String>,
    f_words_inverted: HashMap<i32, String>,
    pub total_iterations: i32,
    pub prob_table: Vec<Vec<f64>>,
    transposed_prob_table: Vec<Vec<f64>>,
    pub highest_prob: HashMap<String, String>,
}

impl IbmModel1 {
    pub fn new(e_corpus_path: String, f_corpus_path: String) -> Self {
        Self {
            e_corpus_path,
            f_corpus_path,
            e_words: HashMap::new(),
            f_words: HashMap::new(),
            e_words_inverted: HashMap::new(),
            f_words_inverted: HashMap::new(),
            total_iterations: 0,
            prob_table: vec![vec![0.0; 0]; 0],
            transposed_prob_table: vec![vec![0.0; 0]; 0],
            highest_prob: HashMap::new(),
        }
    }

    pub fn iterate(&mut self, num_iterations: i32) -> std::io::Result<()> {
        println!("Starting iteration...");
        for epoch in 0..num_iterations {
            let mut e_sentences = BufReader::new(File::open(&mut *self.e_corpus_path)?);
            let mut f_sentences = BufReader::new(File::open(&mut *self.f_corpus_path)?);

            let mut count: HashMap<(String, String), f64> = HashMap::new();
            let mut total: HashMap<String, f64> = HashMap::new();
            let mut total_s: HashMap<String, f64> = HashMap::new();

        for e_f_sentence in e_sentences.by_ref().lines().zip(f_sentences.by_ref().lines()) {
            let e_sentence = e_f_sentence.0?.to_lowercase();
            let e_sentence: Vec<_> = e_sentence.split_whitespace().collect();
            let f_sentence = format!("{} {}", "NULL", e_f_sentence.1?.to_lowercase());
            let f_sentence: Vec<_> = f_sentence.split_whitespace().collect();
            for e_word in e_sentence.iter() {
                for f_word in f_sentence.iter() {
                    total_s.insert(e_word.to_string(), self.prob_table[*self.e_words.get(&e_word.to_string()).unwrap() as usize][*self.f_words.get(&f_word.to_string()).unwrap() as usize] + if total_s.contains_key(&e_word.to_string()) { total_s[&e_word.to_string()] } else { 0.0 });
                }
            }
        }

        let mut e_sentences = BufReader::new(File::open(&mut *self.e_corpus_path)?);
        let mut f_sentences = BufReader::new(File::open(&mut *self.f_corpus_path)?);

        for e_f_sentence in e_sentences.by_ref().lines().zip(f_sentences.by_ref().lines()) {
            let e_sentence = e_f_sentence.0?.to_lowercase();
            let e_sentence: Vec<_> = e_sentence.split_whitespace().collect();
            let f_sentence = format!("{} {}", "NULL", e_f_sentence.1?.to_lowercase());
            let f_sentence: Vec<_> = f_sentence.split_whitespace().collect();
            for e_word in e_sentence.iter() {
                for f_word in f_sentence.iter() {
                    let temp: f64 = self.prob_table[*self.e_words.get(&e_word.to_string()).unwrap() as usize][*self.f_words.get(&f_word.to_string()).unwrap() as usize] / total_s[&e_word.to_string()];
                    count.insert((e_word.to_string(), f_word.to_string()), temp + if count.contains_key(&(e_word.to_string(), f_word.to_string())) { count[&(e_word.to_string(), f_word.to_string())] } else { 0.0 });
                    total.insert(f_word.to_string(), temp + if total.contains_key(&f_word.to_string()) { total[&f_word.to_string()] } else { 0.0 });
                } 
            }
        }

        for (e_word, f_word) in count.keys() {
            self.prob_table[*self.e_words.get(&e_word.to_string()).unwrap() as usize][*self.f_words.get(&f_word.to_string()).unwrap() as usize] = count[&(e_word.to_string(), f_word.to_string())] / total[&f_word.to_string()];
        }
        println!("Iteration {} finished", epoch+1);
    }
    self.transposed_prob_table = transpose(self.prob_table.clone());
    self.total_iterations += num_iterations;
    Ok(())
    }

    pub fn initialize_prob_table(&mut self) -> std::io::Result<()> {
        println!("Initializing probabilities uniformly...");
        let default_value: f64 = 1.0 / self.f_words.len() as f64;
        let default_table = vec![vec![default_value; self.f_words.len()]; self.e_words.len()];
        self.prob_table = default_table;
        Ok(())
    }

    pub fn fill_dict(&mut self) -> std::io::Result<()> {
        let src_input = File::open(&mut *self.f_corpus_path)?;
        let tgt_input = File::open(&mut *self.e_corpus_path)?;
        let mut e_index = 0;
        let mut f_index = 0;
    
        self.f_words.insert(String::from("NULL"), 0);
        f_index += 1;
    
        let src_buffered_reader = BufReader::new(src_input);
        for line in src_buffered_reader.lines() {
            let line = line?.to_lowercase();
            let words = line.split_whitespace();
            for word in words {
                if !self.f_words.contains_key(word) {
                    self.f_words.insert(String::from(word), f_index);
                    f_index += 1;
                }
            }
        }

        let tgt_buffered_reader = BufReader::new(tgt_input);
        for line in tgt_buffered_reader.lines() {
            let line = line?.to_lowercase();
            let words = line.split_whitespace();
            for word in words {
                if !self.e_words.contains_key(word) {
                    self.e_words.insert(String::from(word), e_index);
                    e_index += 1;
                }
            }
        }
        self.e_words_inverted = invert_hashmap(self.e_words.clone());
        self.f_words_inverted = invert_hashmap(self.f_words.clone());
        println!("Initializing finished.");
        Ok(())
    }

    pub fn decode(&self, f_sent: String) -> Vec<String> {
        let mut vec1: Vec<String> = vec![];
        for i in f_sent.split_whitespace() {
            let col_num = self.f_words.get(i);
            match col_num {
                Some(col_num) => {
                    let f_word_col = self.transposed_prob_table[*col_num as usize].clone();
                    let (max_idx, max_val) =
                        f_word_col.iter()
                        .enumerate()
                        .fold((0, f_word_col[0]), |(idx_max, val_max), (idx, val)| {
                            if &val_max > val {
                                (idx_max, val_max)
                            } else {
                                (idx, *val)
                            }
                        });
            let word = self.e_words_inverted.get(&(max_idx as i32));
            vec1.push(word.unwrap().to_string()); 
            },
            None => vec1.push(String::from("<UNK>")),
            };
        }
     return vec1
    }
}

fn invert_hashmap(map: HashMap<String, i32>) -> HashMap<i32, String> {
    let mut invert = HashMap::new();
    for (key, value) in map.into_iter() {
        invert.insert(value, key);
    }
    return invert;
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}