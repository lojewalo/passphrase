#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

mod cli;
mod list;
mod logging;

use crate::list::WordList;

use rand::thread_rng;

const WORD_LIST_JSON: &'static str = include_str!(concat!(env!("OUT_DIR"), "/json"));

lazy_static! {
  static ref WORD_LISTS: Vec<WordList<'static>> = {
    serde_json::from_str(WORD_LIST_JSON).expect("invalid internal json")
  };
}

fn word_list(name: &str) -> Option<&WordList> {
  WORD_LISTS.iter().find(|x| x.short_names.contains(&name))
}

fn main() {
  let args = crate::cli::app(&*WORD_LISTS).get_matches();

  let list_name = args.value_of("list").expect("list value missing");
  let separator = args.value_of("separator").expect("separator value missing");
  let num_words: u8 = match args.value_of("words").expect("words value missing").parse() {
    Ok(x) if x == 0 => {
      eprintln!("must use at least one word");
      return;
    },
    Ok(x) => x,
    Err(e) => {
      eprintln!("invalid amount of words: {}", e);
      return;
    },
  };
  let num_pws: u8 = match args.value_of("passphrases").expect("passphrases value missing").parse() {
    Ok(x) => x,
    Err(e) => {
      eprintln!("invalid amount of passphrases: {}", e);
      return;
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
}
