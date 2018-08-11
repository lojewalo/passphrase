use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct WordList<'a> {
  pub list: HashMap<u32, String>,
  pub short_names: Vec<&'a str>,
  pub name: &'a str,
  pub rolls: u8,
}
