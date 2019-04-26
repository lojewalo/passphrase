use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct WordList {
    pub name: String,
    pub description: String,
    pub short_names: Vec<String>,
    pub rolls: u8,
    pub list: HashMap<u32, String>,
}
