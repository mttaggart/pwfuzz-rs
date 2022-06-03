// pwfuzz-rs [wordlist] [rule] | -r|--rules rules-file -i|--iterations n | -m|--max-chars n
// RULES
// append ! -> PASSWORD!
// prepend !, append 2021 -> !PASSWORD2021
// toggle, append ! -> password!
// insert * 4 -> pass*word
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use rand::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
enum Rule {
    Append(String),
    Prepend(String),
    Upper,
    Lower,
    Insert((String, usize)),
    AppendRandom(usize),
    PrependRandom(usize),
}

#[derive(Serialize, Deserialize, Debug)]
struct RulesDefinition {
    author: String,
    rules: Vec<Rule>,
}

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    wordlist_file: String,

    #[clap(short, long)]
    rules_file: String,

    #[clap(short, long, default_value_t = 1)]
    iterations: usize,
}

fn append(word: &str, appendee: &str) -> String {
    let mut res = String::from(word);
    res.push_str(appendee);
    res
}

fn prepend(word: &str, prependee: &str) -> String {
    let mut res = String::from(prependee);
    res.push_str(word);
    res
}

fn insert(word: &str, insertion: &str, insert_idx: usize) -> String {

    if word.len() > insert_idx + 1 {
        let parts = word.split_at(insert_idx);
        let mut res = String::from(parts.0);
        res.push_str(insertion);
        res.push_str(parts.1);
        res

    } else {
        word.to_string()
    }

}

fn append_random(word: &str, rand_range: usize) -> String {
    let mut rng = rand::thread_rng();
    let rand_n = rng.gen_range(0..rand_range);
    let mut res = String::from(word);
    res.push_str(rand_n.to_string().as_str());
    res
}

fn prepend_random(word: &str, rand_range: usize) -> String {
    let mut rng = rand::thread_rng();
    let rand_n = rng.gen_range(0..rand_range);
    let mut res = rand_n.to_string();
    res.push_str(word);
    res
}

fn apply(rule: &Rule, word: &str) -> String {
    match rule {
        Rule::Append(a) => append(word, a),
        Rule::Prepend(p) => prepend(word, p),
        Rule::Lower => word.to_lowercase(),
        Rule::Upper => word.to_uppercase(),
        Rule::Insert((insertion, insertion_idx)) => insert(word, insertion, *insertion_idx),
        Rule::AppendRandom(rand_range) => append_random(word, *rand_range),
        Rule::PrependRandom(rand_range) => prepend_random(word, *rand_range),
    }
}

fn load_rules<P: AsRef<Path>> (path: P) -> Result<Vec<Rule>, String> {
    match from_str::<RulesDefinition>(read_to_string(path).unwrap_or_default().as_str()) {
        Ok(rules_def) => Ok(rules_def.rules),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    let cli = Cli::parse();

    let mut words = read_to_string(cli.wordlist_file)
        .unwrap_or_else(|_| "".to_string())
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    match load_rules(cli.rules_file) {
        Ok(rules) => {
            let mut new_words: Vec<String> = words.clone();
            for _ in 0..cli.iterations {
                for r in rules.as_slice() {
                    new_words
                        .append(&mut words.iter().map(|w| apply(r, w)).collect::<Vec<String>>());
                }
                words = new_words.clone();
            }

            for n in new_words {
                println!("{n}");
            }
        }
        Err(e) => {
            println!("{e}");
            exit(-1);
        }
    }
}
