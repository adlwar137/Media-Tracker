use crate::media_list::media_entry::*;
use json::JsonValue::Array;
use json::JsonValue::Object;
use std::str::FromStr;
use json::JsonValue;
use json::object;

pub mod media_entry;

#[derive(Debug, Clone)]
pub struct MediaList {
    pub data: Vec<MediaEntry>,
}

impl Default for MediaList {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaList {
    pub fn new() -> Self {
        MediaList {
            data: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, new_entry: MediaEntry) {
        self.data.push(new_entry);
    }
    
    pub fn list_entries(&self) {
        for entry in &self.data {
            println!("Entry:");
            println!("\tName: {}", entry.name);
            println!("\tType: {}", entry.mediaType);
        }
    }
    
    pub fn modify_entry(&mut self, name: &str, new_entry: MediaEntry) {
        self.remove_entry(name);
        self.data.push(new_entry);
    }
    
    pub fn remove_entry(&mut self, name: &str) {
        self.data = self.data.iter().filter(|s| s.name != name).cloned().collect();
    }
}

impl From<JsonValue> for MediaList {
    fn from(value: JsonValue) -> Self {
        let mut data = MediaList::new();

        if let Array(vector) = value {
            for entry in vector {
                if let Object(object) = entry {
                    let mtype = MediaType::from_str(&object["mediaType"].to_string()).expect("wrong type");
                    data.data.push(MediaEntry {
                        name: object["name"].to_string(),
                        mediaType: mtype,
                    });
                }
            }
        } else {
            panic!("wrong value");
        }

        data

    }
}

impl From<MediaList> for JsonValue {
    fn from(val: MediaList) -> Self {
        let mut the_array = json::JsonValue::new_array();

        for entry in val.data {
            let wrapped_entry = object!{
                name: entry.name,
                mediaType: entry.mediaType.to_string(),
            };

            the_array.push(wrapped_entry).expect("the fuck you say about me");
        }

        the_array
   }
}
