use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[derive(Parser)]
#[command(version)]
/// Quick lookup using Merriam-Webster Collegiate Dictionary API.
pub struct Args {
    /// Word to be looked up.
    word: String,

    /// Output raw text.
    #[arg(short, long)]
    raw: bool,

    /// Pass API key.
    #[arg(long)]
    api_key: Option<String>,
}

pub async fn core_fn(args: Args) -> Result<()> {
    let api_key = match args.api_key {
        Some(key) => key,
        None => env::var("MW_API_KEY").with_context(|| "MW_API_KEY is not set!")?,
    };
    let req_url = format!(
        "https://www.dictionaryapi.com/api/v3/references/collegiate/json/{word}?key={api_key}",
        word = args.word
    );
    let response = reqwest::get(&req_url).await?;

    if !args.raw {
        let entries: Entries = response.json().await?;
        for entry in entries {
            entry.print();
        }
    } else {
        println!("{}", response.text().await?);
    }

    Ok(())
}

pub type Entries = Vec<Entry>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    meta: Meta,
    hom: Option<i64>,
    hwi: HeadwordInfo,
    fl: Option<String>,
    def: Option<Value>,
    date: Option<String>,
    shortdef: Vec<String>,
    lbs: Option<Vec<String>>,
    uros: Option<Value>,
    dros: Option<Value>,
    et: Option<Value>,
    ins: Option<Value>,
    cxs: Option<Value>,
    usages: Option<Value>,
    dxnls: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    id: String,
    uuid: String,
    sort: String,
    src: String,
    section: String,
    stems: Vec<String>,
    offensive: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct HeadwordInfo {
    hw: String,
    prs: Option<Value>,
}

fn number_each_item(item: (usize, &String)) -> String {
    let mut n = format!("{}. ", item.0 + 1);
    n.push_str(item.1);

    n
}

impl Entry {
    pub fn short_def(&self) -> String {
        let numbered_defs: Vec<String> = self
            .shortdef
            .iter()
            .enumerate()
            .map(|x| number_each_item(x))
            .collect();

        numbered_defs.join("\n")
    }

    pub fn print(&self) {
        let short_def = self.short_def();
        let stems = self.meta.stems.join(", ");

        println!(
            "\x1b[1;32m{headword}\x1b[0m\n\x1b[4m{pos}.\x1b[0m ({stems})",
            headword = self.hwi.hw,
            pos = match &self.fl {
                Some(fl) => fl,
                None => "null",
            }
        );
        println!("{short_def}");
    }
}
