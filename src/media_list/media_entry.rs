use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone)]
pub enum MediaType {
    Movie,
    Book,
    Show,
}

pub enum MediaProgress {
    Movie(bool),
    Show(i32, i32),
}

#[derive(Debug, Clone)]
pub struct MediaEntry {
   pub mediaType: MediaType,
   pub name: String,
}

impl MediaProgress {
    pub fn get_progress(&self) -> f32 {
        match self {
            Movie(x) => {
                if x {
                    100.0
                } else {
                    0.0
                }
            },
            Show(x, y) => {
                x/y
            }
        }
    }
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

