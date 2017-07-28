#[derive(Clone)]
pub struct Channel {
    raw: String,
    parts: Vec<String>,
}

impl ::std::fmt::Debug for Channel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "Channel({})", self.to_string())
    }
}

impl<'a> From<&'a String> for Channel {
    fn from(str: &String) -> Channel {
        Channel {
            raw: str.clone(),
            parts: str.split('.').map(String::from).collect(),
        }
    }
}
impl<'a> From<&'a str> for Channel {
    fn from(str: &str) -> Channel {
        Channel {
            raw: String::from(str),
            parts: str.split('.').map(String::from).collect(),
        }
    }
}
impl<'a> From<&'a &'a str> for Channel {
    fn from(str: &&str) -> Channel {
        Channel {
            raw: String::from(*str),
            parts: str.split('.').map(String::from).collect(),
        }
    }
}
impl From<String> for Channel {
    fn from(str: String) -> Channel {
        Channel {
            parts: str.split('.').map(String::from).collect(),
            raw: str.clone(),
        }
    }
}

impl ToString for Channel {
    fn to_string(&self) -> String {
        self.raw.clone()
    }
}

impl Channel {
    pub fn matches(&self, other: &Channel) -> bool {
        let mut last_was_wildcard = false;
        for i in 0..self.parts.len() {
            if self.parts[i] == "*" {
                last_was_wildcard = true;
                continue;
            }
            if i >= other.parts.len() {
                return false;
            }
            if self.parts[i] != other.parts[i] {
                return false;
            }
        }
        // if the last token was a wildcard, accept any parts after it
        // if the two channels have the same length and we got here, they match
        last_was_wildcard || self.parts.len() == other.parts.len()
    }

    pub fn is(&self, other: &str) -> bool {
        self.raw == other
    }
}
