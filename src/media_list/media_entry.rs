use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone)]
pub enum MediaType {
    Movie,
    Book,
    Show,
}

// TODO makes this a proper error definition
#[derive(Debug)]
pub struct FunneError;

impl FromStr for MediaType {
    type Err = FunneError;
    
    fn from_str(s: &str) -> Result<MediaType, Self::Err> {
        match s.to_lowercase().as_str() {
            "movie" => Ok(MediaType::Movie),
            "book" => Ok(MediaType::Book),
            "show" => Ok(MediaType::Show),
            _ => Err(FunneError),
        }
    }
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MediaType::Movie => write!(f, "Movie"),
            MediaType::Book => write!(f, "Book"),
            MediaType::Show => write!(f, "Show"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MediaEntry {
   pub mediaType: MediaType,
   pub name: String,
}

impl MediaEntry {
    pub fn search(list: &Vec<Self>, query: &str) -> Option<Self> {
        for entry in list {
            if entry.name == query.to_lowercase().as_str() {
                return Some(entry.clone());
            }
        }
        None
    }
}
