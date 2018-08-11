#![feature(rust_2018_preview, process_exitcode_placeholder, use_extern_macros)]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

mod cli;
mod list;
mod logging;

use crate::list::WordList;

use lazy_static::lazy_static;

use rand::thread_rng;

use std::process::ExitCode;

const WORD_LIST_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/json"));

lazy_static! {
  static ref WORD_LISTS: Vec<WordList<'static>> = {
    serde_json::from_str(WORD_LIST_JSON).expect("invalid internal json")
  };
}

fn word_list(name: &str) -> Option<&WordList> {
  WORD_LISTS.iter().find(|x| x.short_names.contains(&name))
}

fn main() -> ExitCode {
  let args = crate::cli::app(&*WORD_LISTS).get_matches();

  let list_name = args.value_of("list").expect("list value missing");
  let separator = args.value_of("separator").expect("separator value missing");
  let num_words: u8 = match args.value_of("words").expect("words value missing").parse() {
    Ok(x) if x == 0 => {
      eprintln!("must use at least one word");
      return ExitCode::FAILURE;
    },
    Ok(x) => x,
    Err(e) => {
      eprintln!("invalid amount of words: {}", e);
      return ExitCode::FAILURE;
    },
  };
  let num_pws: usize = match args.value_of("passphrases").expect("passphrases value missing").parse() {
    Ok(x) => x,
    Err(e) => {
      eprintln!("invalid amount of passphrases: {}", e);
      return ExitCode::FAILURE;
    },
  };

  if args.is_present("verbose") {
    crate::logging::set_up();
  }

  let list = word_list(list_name).unwrap();
  let mut rng = thread_rng();

  let isnt_tty = atty::isnt(atty::Stream::Stdout);

  for i in 0..num_pws {
    let passphrase = list.generate(&mut rng, num_words, separator);

    if isnt_tty && i == num_pws - 1 {
      print!("{}", passphrase);
    } else {
      println!("{}", passphrase);
    }
  }

  ExitCode::SUCCESS
}
