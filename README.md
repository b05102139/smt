# Computational Linguistics Team Lab Project
This repo contains code written for the CL Team Lab project at the Institute of Natural Language Processing, University of Stuttgart.

It contains a Rust implementation of IBM Model 1, along with Python implementations of byte-pair encoding and BLEU for tokenization and evaluation.

# Baseline Model - IBM-Model 1:

To build the Rust code, we assume that Rust and Cargo is already installed. Please follow the instructions on [this page](https://www.rust-lang.org/tools/install), which details how to use rustup to download Rust, and will have Cargo installed alongside automatically.

Assuming that is done, in order to compile the code, please run:

```
cargo build
```

Which builds the project. After the project is build, the code can be run while in the main directory:

```
./target/debug/IBM-1
```

The source and target corpora are to be specified in the main.rs file under src/, which should be parallel corpora of the two languages, each put in their own .txt file.

# Code in Paper - Word Segmentation:

For the segmentation process

# Code in Paper - Moses Training:

## 1. Install Giza++ and Moses

### make a directory for moses and giza++
```bash
mkdir smt
cd smt
```

### install several packages required
```bash
sudo apt-get install build-essential git-core pkg-config automake libtool wget zlib1g-dev python-dev libbz2-dev
sudo apt-get install libsoap-lite-perl
```


### clone Giza++ from Github
```bash
git clone https://github.com/moses-smt/giza-pp.git
```


### Enter to Giza++ directory and install the 
```bash
package
cd giza-pp
make
```

### copying Giza++ Binaries to "tools" directory
```bash
cd ..
mkdir tools
cp giza-pp/GIZA++-v2/GIZA++ giza-pp/GIZA++-v2/snt2cooc.out giza-pp/mkcls-v2/mkcls tools
```


### download pre-built binary files of Moses and related tools
```bash
cd smt
wget https://www.achrafothman.net/aslsmt/tools/smt-moses-ao-ubuntu-16.04.tgz
tar -xzvf smt-moses-ao-ubuntu-16.04.tgz
cd ubuntu-16.04/
mv bin ../
mv scripts ../
mv training-tools ../
cd ..
rm -r ubuntu-16.04
rm -r smt-moses-ao-ubuntu-16.04.tgz
```

### prepare corpus for training, test, and tuning in corpus folder
```bash
mkdir corpus
cd corpus
ls
> test.ko  test.zh  train.ko  train.zh  tuning.ko  tuning.zh to this folder
```



## 2. Training an MT model
### Train a language model for target language
```bash
/home/dojun/smt/bin/lmplz -o 3 < /home/dojun/smt/corpus/train.ko > /home/dojun/smt/corpus/train.arpa.ko
```

### Binarise the LM
```bash
/home/dojun/smt/bin/build_binary /home/dojun/smt/corpus/train.arpa.ko /home/dojun/smt/corpus/train.blm.ko
```


### How to test the LM?
```bash
echo "안녕 하세요" | /home/dojun/smt/bin/query /home/dojun/smt/corpus/train.blm.ko
```

### train work alignment, phrase extraction, scoring, reordering table, create Moses config file
```bash
mkdir working
cd working  
nohup nice /home/dojun/smt/scripts/training/train-model.perl -root-dir train -corpus /home/dojun/smt/corpus/train -f zh -e ko -alignment grow-diag-final-and -reordering msd-bidirectional-fe -lm 0:3:/home/dojun/smt/corpus/train.blm.ko:8 -external-bin-dir /home/dojun/smt/tools >& training.out &
tail -f training.out 
   # once the line starting with "(9) create moses.ini @..." appears, you can type CTRL+C to exit the tail mode.
cd ..
```


### Tuning
```bash
cd ..
cd working
nohup nice /home/dojun/smt/scripts/training/mert-moses.pl /home/dojun/smt/corpus/tuning.zh /home/dojun/smt/corpus/tuning.ko /home/dojun/smt/bin/moses /home/dojun/smt/working/train/model/moses.ini --mertdir /home/dojun/smt/bin/ &> mert.out &
tail -f mert.out
    # once the line starting with "Saving new config to: ./moses.ini Saved: ./moses.ini..." appears, you can type CTRL+C to exit the tail mode.
cd ..
```

### Binarise the phrase-table and lexicalized reordering models
```bash
```
```bash
cd working
mkdir binarised-model
/home/dojun/smt/bin/processPhraseTableMin -in train/model/phrase-table.gz -nscores 4 -out binarised-model/phrase-table
/home/dojun/smt/bin/processLexicalTableMin -in train/model/reordering-table.wbe-msd-bidirectional-fe.gz -out binarised-model/reordering-table
cp /home/dojun/smt/working/mert-work/moses.ini /home/dojun/smt/working/binarised-model/
cd binarised-model/
vim moses.ini
```

### in the vim editor, type i to start editing mode and edit the following
```bash
        @1. Change PhraseDictionaryMemory to PhraseDictionaryCompact
        @2. Set the path of the PhraseDictionaryCompact feature to point to: /home/dojun/smt/working/binarised-model/phrase-table.minphr
        @3. Set the path of the LexicalReordering feature to point to: /home/dojun/smt/working/binarised-model/reordering-table
        @4. Save moses.ini
        # to save and quit, type ESC and type wq! followed by ENTER.

```

### Translation
```bash
cd ..
cd working
/home/dojun/smt/scripts/training/filter-model-given-input.pl filtered-corpus-mini mert-work/moses.ini /home/dojun/smt/corpus/test.zh -Binarizer /home/dojun/smt/bin/processPhraseTableMin
nohup nice /home/dojun/smt/bin/moses -f /home/dojun/smt/working/filtered-corpus-mini/moses.ini < /home/dojun/smt/corpus/test.zh > /home/dojun/smt/working/test.translated.ko 2> /home/dojun/smt/working/test.translated.out
    # See the log
    tail -f /home/dojun/smt/working/test.translated.out
    cd ..
```

### Calculate BLEU 
```bash
/home/dojun/smt/scripts/generic/multi-bleu.perl -lc /home/dojun/smt/corpus/test.ko < /home/dojun/smt/working/test.translated.ko
```

## Reference
Thanks to [achrafothman.net](https://achrafothman.net/site/category/tutorials/) for this nice Moses tutorial

Team members: (Ryan) Soh-Eun Shim, Dojun Park