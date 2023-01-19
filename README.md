# rust-ml-inference
Playing around with Rust and comparing ML inference between rust and python :crab: :snake:

## Rust Wikipedia Summarization Tool
A command line tool to collect and summarize Wikipedia pages. Built using [rust-bert](https://github.com/guillaume-be/rust-bert) a [torch-rs](https://github.com/LaurentMazare/tch-rs) based Rust wrapper for Huggingface transformer models.
To use the tool `cd` into the 'rust_wiki_summarization' directory and run `cargo build --release` and then
```rs
cargo run -- -p "page 1" -p "page 2" ... -p "page n"
```
For example, `cargo run -- -p rust -p python -p "Graydon Hoare"` return summaries return summaries for rust, python, and Graydon Hoare (the creator of rust!). Note you do not have to run `cargo build` and can just run `run` directly and it will still build the binaries. The nitial code for this section was taken and adapted from Noah Gift's [rust mlops template](https://github.com/noahgift/rust-mlops-template).
