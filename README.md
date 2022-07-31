# Computational Linguistics Team Lab Project
This repo contains code written for the CL Team Lab project at the Institute of Natural Language Processing, University of Stuttgart.

It contains a Rust implementation of IBM Model 1, along with Python implementations of byte-pair encoding and BLEU for tokenization and evaluation.

IBM-Model 1:

To build the Rust code, we assume that Rust and Cargo is already installed. Please follow the instructions on [this page](https://www.rust-lang.org/tools/install), which details how to use rustup to download Rust, and will have Cargo installed alongside automatically.

Assuming that is done, in order to compile the code, please run:

```
cargo build
```

Which builds the project. After the project is build, the code can be run while in the main directory:

```
./target/debug/IBM-1
```

The source and target corpora are to be specified in the main.rs file under src/, which should be parallel corpora of the two languages, each put in their own txt file.

Team members: (Ryan) Soh-Eun Shim, Dojun Park