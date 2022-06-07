use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::string::String;
use ndarray::{Array, Array2, Axis};

pub struct IbmModel1 {
    pub e_corpus_path: String,
    pub f_corpus_path: String,
    pub e_words: HashMap<String, i32>,
    pub f_words: HashMap<String, i32>,
    e_words_inverted: HashMap<i32, String>,
    f_words_inverted: HashMap<i32, String>,
    pub total_iterations: i32,
    pub prob_table: Array2::<f64>,
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
            prob_table: Array2::<f64>::zeros((3,3)),
            highest_prob: HashMap::new(),
        }
    }

    pub fn iterate(&mut self, num_iterations: i32, delta: f64) -> std::io::Result<()> {
        let mut previous_table:Array2::<f64> = self.prob_table.clone();
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
            let mut f_sentence: Vec<_> = f_sentence.split_whitespace().collect();
            for e_word in e_sentence.iter() {
                for f_word in f_sentence.iter() {
                    total_s.insert(e_word.to_string(), self.prob_table[[*self.e_words.get(&e_word.to_string()).unwrap() as usize, *self.f_words.get(&f_word.to_string()).unwrap() as usize]] + if total_s.contains_key(&e_word.to_string()) { total_s[&e_word.to_string()] } else { 0.0 });
                }
            }
        }

        let mut e_sentences = BufReader::new(File::open(&mut *self.e_corpus_path)?);
        let mut f_sentences = BufReader::new(File::open(&mut *self.f_corpus_path)?);

        for e_f_sentence in e_sentences.by_ref().lines().zip(f_sentences.by_ref().lines()) {
            let e_sentence = e_f_sentence.0?.to_lowercase();
            let e_sentence: Vec<_> = e_sentence.split_whitespace().collect();
            let f_sentence = format!("{} {}", "NULL", e_f_sentence.1?.to_lowercase());
            let mut f_sentence: Vec<_> = f_sentence.split_whitespace().collect();
            for e_word in e_sentence.iter() {
                for f_word in f_sentence.iter() {
                    let temp: f64 = self.prob_table[[*self.e_words.get(&e_word.to_string()).unwrap() as usize, *self.f_words.get(&f_word.to_string()).unwrap() as usize]] / total_s[&e_word.to_string()];
                    count.insert((e_word.to_string(), f_word.to_string()), temp + if count.contains_key(&(e_word.to_string(), f_word.to_string())) { count[&(e_word.to_string(), f_word.to_string())] } else { 0.0 });
                    total.insert(f_word.to_string(), temp + if total.contains_key(&f_word.to_string()) { total[&f_word.to_string()] } else { 0.0 });
                } 
            }
        }

        for (e_word, f_word) in count.keys() {
            self.prob_table[[*self.e_words.get(&e_word.to_string()).unwrap() as usize, *self.f_words.get(&f_word.to_string()).unwrap() as usize]] = count[&(e_word.to_string(), f_word.to_string())] / total[&f_word.to_string()];
        }
        println!("Delta: {}", table_distance(&self.prob_table, &previous_table));
        if delta > table_distance(&self.prob_table, &previous_table) {
            println!("Tables have converged under the specified delta.");
            break
        }
        previous_table = self.prob_table.clone();
        println!("Iteration {} finished", epoch+1);
    }
    self.total_iterations += num_iterations;
    Ok(())
    }

    pub fn initialize_prob_table(&mut self) -> std::io::Result<()> {
        println!("Initializing probabilities uniformly...");
        let default_value: f64 = 1.0 / self.f_words.len() as f64;
        let default_table = Array::from_elem((self.e_words.len(), self.f_words.len()), default_value);
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
            let word = self.highest_prob.get(i);
            match word {
                Some(word) => {
                    vec1.push(word.to_string()); 
                },
                None => vec1.push(String::from("<UNK>")),
            };
        }
        return vec1
    }

    pub fn cache_translations(&mut self) {
        for (i, row) in self.prob_table.axis_iter(Axis(1)).enumerate() {
            let (max_idx, max_val) =
                row.iter()
                    .enumerate()
                    .fold((0, row[0]), |(idx_max, val_max), (idx, val)| {
                        if &val_max > val {
                            (idx_max, val_max)
                        } else {
                            (idx, *val)
                        }
                    });
                    self.highest_prob.insert(self.f_words_inverted[&(i as i32)].clone(), self.e_words_inverted[&(max_idx as i32)].clone());
        }
    }
}

fn invert_hashmap(map: HashMap<String, i32>) -> HashMap<i32, String> {
    let mut invert = HashMap::new();
    for (key, value) in map.into_iter() {
        invert.insert(value, key);
    }
    return invert;
}

fn table_distance(current_table: &Array2::<f64>, previous_table: &Array2::<f64>) -> f64 {
    let zipped = current_table.into_iter().zip(previous_table.into_iter());
    let mut sum_of_dist: f64 = 0.0;
    for i in zipped {
        sum_of_dist += (i.0 - i.1).abs();
    }
    return sum_of_dist;
}