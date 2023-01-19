use anyhow::Result;
use rust_bert::pipelines::summarization::SummarizationModel;
use std::{collections::HashMap, vec::Vec};
use wikipedia;

pub enum WikiEntry {
    PageContent(String),
    NoPageFound(String),
}

// get Wikipedia context given page title
pub fn get_wiki_content(page_title: &String) -> WikiEntry {
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    let page = wiki.page_from_title(page_title.to_owned());
    let content = page.get_content();
    let output = match content {
        Ok(page_content) => WikiEntry::PageContent(page_content),
        Err(..) => {
            WikiEntry::NoPageFound(format!("No page wiki page '{page_title}' found").to_string())
        }
    };
    output
}

// get Wikipedia content from multiple pages
pub fn get_wiki_contents(page_titles: &Vec<String>) -> HashMap<String, WikiEntry> {
    let mut contents: HashMap<String, WikiEntry> = HashMap::new();
    for page in page_titles {
        contents.insert(page.clone(), get_wiki_content(&page));
    }
    contents
}

// load summarization model
pub fn load_model() -> Result<SummarizationModel> {
    let model = SummarizationModel::new(Default::default())?;
    Ok(model)
}

// Summarizes the content from a wikipedia page
pub fn summarize_content(model: &SummarizationModel, content: &WikiEntry) -> String {
    match content {
        WikiEntry::PageContent(s) => {
            // preprocess input by capping char count to 500 and removing new lines
            let input = s.chars().take(500).collect::<String>().replace('\n', " ");
            let input = vec![input];
            let output = model.summarize(&input);
            output[0].clone()
        }
        WikiEntry::NoPageFound(s) => s.clone(),
    }
}

// Get summaries of a list of wikipedia pages
pub fn get_summaries(pages: &Vec<String>) -> Result<HashMap<String, String>> {
    let model = load_model()?;
    let contents: HashMap<String, WikiEntry> = get_wiki_contents(&pages);
    let mut summaries: HashMap<String, String> = HashMap::new();
    for (page_title, content) in contents.iter() {
        let summary = summarize_content(&model, content);
        summaries.insert(page_title.clone(), summary);
    }
    Ok(summaries)
}
