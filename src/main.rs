mod IbmModel1;

use std::fs::File;
use std::io::{Read, BufRead, BufReader, BufWriter, Write};

fn main() {
    // initialize model with path to training data
    let mut myMod = IbmModel1::IbmModel1::new(String::from("data/en_train.txt"), String::from("data/fr_train.txt"));
    // fill up word-index dictionary
    let _res = myMod.fill_dict();
    // Initialize translation probability uniformly 
    let _res = myMod.initialize_prob_table();
    // set the desired number of iterations
    let _res = myMod.iterate(3);

    // Create output file
    let f = File::create("fr_en_test_output.txt").expect("unable to create file");
    // Write output to it
    let mut f = BufWriter::new(f);
    let mut input = BufReader::new(File::open("data/fr_test.txt").unwrap());
    for i in input.by_ref().lines() {
        let decoded = myMod.decode(i.unwrap()).join(" ");
        println!("{}", decoded);
        writeln!(f, "{}", decoded);
    }
}
