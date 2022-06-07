use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

mod IbmModel1;

fn main() {
    let mut myMod = IbmModel1::new(String::from("data/en_train.txt"), String::from("data/fr_train.txt"));
    let _res = myMod.fill_dict();
    let _res = myMod.initialize_prob_table();
    let _res = myMod.iterate(3, 0.05);
    let _res = myMod.cache_translations();
    let f = File::create("fr_en_test_output.txt").expect("unable to create file");
    let mut f = BufWriter::new(f);
    let mut input = BufReader::new(File::open("data/fr_test.txt")?);

    for i in input.by_ref().lines() {
        let decoded = myMod.decode(i.unwrap()).join(" ");
        println!("{}", decoded);
        writeln!(f, "{}", decoded);
    }

    let mut myMod = IbmModel1::new(String::from("data/ko_train.txt"), String::from("data/zh_train.txt"));
    let _res = myMod.fill_dict();
    let _res = myMod.initialize_prob_table();
    let _res = myMod.iterate(3, 0.05);
    let _res = myMod.cache_translations();
    let f = File::create("ko_zh_test_output.txt").expect("unable to create file");
    let mut f = BufWriter::new(f);
    let mut input = BufReader::new(File::open("data/zh_test.txt")?);
    let mut res = 0;
    for i in input.by_ref().lines() {
        let decoded = myMod.decode(i.unwrap()).join(" ");
        write!(f, "{}\n", decoded);
        println!("{}", decoded);
        res += 1;
    }

}
