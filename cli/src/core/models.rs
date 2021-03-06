use serde::{Deserialize, Deserializer};
use serde_json;
use decoding::text_decoding::decode_html;

#[derive(Serialize)]
pub struct HnListOfItems {
    pub values: Vec<i32>,
}

impl<'de> Deserialize<'de> for HnListOfItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        Deserialize::deserialize(deserializer).map(|arr: Vec<i32>| HnListOfItems { values: arr })
    }
}

fn default_user() -> String {
    String::from("Undefined user")
}

#[derive(Serialize, Deserialize)]
pub struct HnItem {
    #[serde(default = "default_user")]
    pub by: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent: Option<i32>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub descendants: Option<i32>,
    pub id: i32,
    #[serde(skip_serializing_if="Option::is_none")]
    pub kids: Option<Vec<i32>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub score: Option<i32>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub text: Option<String>,
    pub time: f64,
    #[serde(rename(deserialize = "type"))]
    pub type_str: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub dead: Option<bool>,
}

pub struct HnItemCommentMap {
    pub parent: HnItem,
    pub comments: Vec<HnItem>,
    pub depth: usize,
}

impl HnItem {
    pub fn text_unescaped(&self) -> Option<String> {
        if self.text.is_some() {
            let unescaped = decode_html(self.text.as_ref().unwrap());
            if unescaped.is_ok() {
                let unwrapped = unescaped.unwrap();
                return Some(unwrapped);
            }
        }
        None
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct HnUser {
    pub about: String,
    pub created: f64,
    pub id: String,
    pub karma: i32,
    pub submitted: Vec<i32>,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hn_item_serde_test() {
        use std::fs::File;
        use std::io::prelude::*;
        let mut contents = String::new();
        File::open("res/test/item.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnItem = serde_json::from_str(&contents).unwrap();
        assert_eq!(71, deserialized.descendants.unwrap());
        assert_eq!("dhouston", deserialized.by);
        assert_eq!(8863, deserialized.id);
        assert_eq!(111, deserialized.score.unwrap());
        assert_eq!(1175714200.0f64, deserialized.time);
        assert_eq!("My YC app: Dropbox - Throw away your USB drive",
                   deserialized.title.unwrap());
        assert_eq!("story", deserialized.type_str);
        assert_eq!("http://www.getdropbox.com/u/2/screencast.html",
                   deserialized.url.unwrap());
    }

    #[test]
    fn hn_item_decode_test() {
        use std::fs::File;
        use std::io::prelude::*;
        let mut contents = String::new();
        File::open("res/test/children-item.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnItem = serde_json::from_str(&contents).unwrap();
        let text_decoded = deserialized.text_unescaped().unwrap();
        println!("{}", text_decoded);
        assert!(!text_decoded.contains("&#x2F;"));
    }

    #[test]
    fn hn_top_stories_serde_test() {
        use std::fs::File;
        use std::io::prelude::*;
        let mut contents = String::new();
        File::open("res/test/top-stories.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnListOfItems = serde_json::from_str(&contents).unwrap();
        assert!(deserialized.values.len() > 3);
    }
    #[test]
    fn hn_user_serde_test() {
        use std::fs::File;
        use std::io::prelude::*;
        let mut contents = String::new();
        File::open("res/test/user.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnUser = serde_json::from_str(&contents).unwrap();
        assert_eq!("This is a test", deserialized.about);
        assert_eq!(1173923446.0f64, deserialized.created);
        assert_eq!("jl", deserialized.id);
        assert_eq!(3496, deserialized.karma);
        assert!(deserialized.submitted.len() > 3);
    }

    #[test]
    fn dead_hnitem() {
        use std::fs::File;
        use std::io::prelude::*;
        let mut contents = String::new();
        File::open("res/test/dead-item.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnItem = serde_json::from_str(&contents).unwrap();
        assert_eq!(true, deserialized.dead.unwrap());
    }
}