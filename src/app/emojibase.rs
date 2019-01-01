use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EmojiBase {
  pub tags: HashMap<String, String>,
  pub categories: HashMap<String, Vec<String>>
}

pub fn find_emojis(emojis: &EmojiBase, search_phrase: String) -> Vec<String> {
  let mut result : Vec<String> = Vec::new();
  if emojis.categories.contains_key(&search_phrase) {
    let category : &Vec<String> = emojis.categories.get(&search_phrase).expect("");
    for emoji in category {
      result.push(emoji.to_string());
    }
  } else {
    let tags = &emojis.tags;
    let lower_search_phrase = search_phrase.to_lowercase();
    for tag in tags {
      let tag_lower = tag.0.to_lowercase();
      if tag_lower.contains(&lower_search_phrase) {
        result.push(tag.1.to_string());
      }
    }
  }
  return result;
}

pub fn load_emojibase() -> EmojiBase{
  let contents = fs::read_to_string("emojis.json").expect("Unable to find emojis.json");
  let emojibase_decoded = json::parse(&contents).unwrap();
  let mut _tags : HashMap<String, String> = HashMap::new();
  let mut _categories : HashMap<String, Vec<String>> = HashMap::new();
  for property in emojibase_decoded.entries(){
    if property.0 == "tags" {
      for tag in property.1.entries(){
        let key = tag.0;
        let v = tag.1.as_str().expect("Emoji should be stored as a string");
        _tags.insert(key.to_string(), v.to_string());
      }
    }
    if property.0 == "categories" {
      for category in property.1.entries() {
        let key = category.0;
        let mut v : Vec<String> = Vec::new();
        if category.1.is_array() {
          for entry in category.1.members() {
            v.push(entry.as_str().expect("Emoji should be stored as a string").to_string());
          }
        } else {
          panic!("Emoji category should correspond to an array of emojis");
        }
        _categories.insert(key.to_string(), v);
      }
    }
  }
  return EmojiBase{categories: _categories, tags: _tags};
}