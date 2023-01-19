use clap::Parser;
use rust_wiki_summarization::get_summaries;
use std::vec::Vec;

// A command-line tool to summarize wikipedia content given a list of pages
#[derive(Parser, Debug)]
#[command(
    author = "Jack Leitch",
    version = "1.0",
    about = "A command-line tool to summarize wikipedia content given a list of pages"
)]
struct Args {
    // Page title of wikipedia page that we want a summary for
    #[arg(short, long, action = clap::ArgAction::Append)]
    page: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();
    match args.page {
        Some(p) => {
            let summaries = get_summaries(&p).expect("Failed to load model!");
            for (page, summary) in summaries.iter() {
                println!("{}: \n", page);
                println!("{}: \n", summary);
            }
        }
        None => print!("Pleas give page titles by using '-p page_name'"),
    }
}
