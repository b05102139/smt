mod IbmModel1;

use std::fs::File;
use std::io::{Read, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut myMod = IbmModel1::IbmModel1::new(String::from("data/en_train.txt"), String::from("data/fr_train.txt"));
    let _res = myMod.fill_dict();
    let _res = myMod.initialize_prob_table();
    let _res = myMod.iterate(3);

    let f = File::create("fr_en_test_output.txt").expect("unable to create file");
    let mut f = BufWriter::new(f);
    let mut input = BufReader::new(File::open("data/fr_test.txt").unwrap());
    for i in input.by_ref().lines() {
        let decoded = myMod.decode(i.unwrap()).join(" ");
        println!("{}", decoded);
        writeln!(f, "{}", decoded);
    }
}
