#[macro_use]
extern crate serde_derive;

use std::{
  collections::HashMap,
  fs::{self, File},
  path::Path,
};

// read the json into the build program
const METADATA_JSON: &'static str = include_str!(concat!(
  env!("CARGO_MANIFEST_DIR"),
  "/lists/metadata.json",
));

// metadata format structs

#[derive(Debug, Deserialize)]
struct Metadata<'a> {
  #[serde(borrow)]
  lists: Vec<List<'a>>,
}

#[derive(Debug, Deserialize)]
struct List<'a> {
  #[serde(borrow)]
  name: &'a str,
  #[serde(borrow)]
  short_names: Vec<&'a str>,
  #[serde(borrow)]
  file: &'a str,
  rolls: u8,
}

// here we include the wordlist model from the actual program

mod list {
  include!("src/list/model.rs");
}

fn main() {
  // parse the json
  let data: Metadata = serde_json::from_str(METADATA_JSON).expect("invalid metadata.json");

  // create the path to <repo root>/lists
  let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("lists");

  // create a list of word lists
  let mut lists = Vec::with_capacity(data.lists.len());

  for list in data.lists {
    // read in the raw word list
    let data = fs::read_to_string(manifest.join(list.file)).unwrap();
    // parse the raw word list
    let words = parse(&data);

    let l = list::WordList {
      list: words,
      name: list.name,
      short_names: list.short_names,
      rolls: list.rolls,
    };
    lists.push(l);
  }

  // create the file that will be included in the actual program
  let json_path = Path::new(&std::env::var("OUT_DIR").unwrap()).join("json");
  let json_file = File::create(&json_path).unwrap();

  // write out the json
  serde_json::to_writer(json_file, &lists).unwrap();
}

fn parse(content: &str) -> HashMap<u32, String> {
  content
    .split('\n')
    .filter(|x| !x.is_empty())
    .map(|x| x.split('\t'))
    .map(|mut x| (
      // parse the first segment as a u32
      x.next().unwrap().parse::<u32>().unwrap(),
      // parse the second segment as a string
      x.next().unwrap().to_string(),
    ))
    .collect()
}