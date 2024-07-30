use crate::media_list::media_entry::*;
use json::JsonValue;
use json::object;

pub mod media_entry;

#[derive(Debug, Clone)]
pub struct MediaList {
    pub data: Vec<MediaEntry>,
}

impl MediaList {
    // fn add_entry(mut self) {
    
    //     let entry = MediaEntry {
    //         mediaType: MediaType::from_str(&media).expect("dass da wrong numbaaa"),
    //         name,
    //     };
    
    //     entryList.push(entry);
    // }

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
        todo!("well shit");    
    }
}

impl Into<JsonValue> for MediaList {
    fn into(self) -> JsonValue {
        let mut the_array = json::JsonValue::new_array();

        for entry in self.data {
            let wrapped_entry = object!{
                name: entry.name,
                mediaType: entry.mediaType.to_string(),
            };

            the_array.push(wrapped_entry).expect("the fuck you say about me");
        }

        the_array
   }
}
