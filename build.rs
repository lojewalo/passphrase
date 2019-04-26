#[macro_use]
extern crate serde_derive;

use serde::Serialize;

use xz2::write::XzEncoder;

use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
};

// read the json into the build program
const METADATA_JSON: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/lists/metadata.json",));

// metadata format structs

#[derive(Debug, Deserialize)]
struct Metadata {
    lists: Vec<List>,
}

#[derive(Debug, Deserialize)]
struct List {
    name: String,
    description: String,
    short_names: Vec<String>,
    file: String,
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
            name: list.name,
            description: list.description,
            short_names: list.short_names,
            rolls: list.rolls,
            list: words,
        };
        lists.push(l);
    }

    // create the file that will be included in the actual program
    let bytes_path = Path::new(&std::env::var("OUT_DIR").unwrap()).join("bytes");
    let bytes_file = File::create(&bytes_path).unwrap();

    // write out the data
    lists
        .serialize(&mut rmp_serde::encode::Serializer::new(XzEncoder::new(
            bytes_file, 9,
        )))
        .unwrap();
}

fn parse(content: &str) -> HashMap<u32, String> {
    content
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.split('\t'))
        .map(|mut x| {
            (
                // parse the first segment as a u32
                x.next().unwrap().parse::<u32>().unwrap(),
                // parse the second segment as a string
                x.next().unwrap().to_string(),
            )
        })
        .collect()
}
