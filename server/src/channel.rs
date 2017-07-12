pub struct Channel {
    // raw: String,
    parts: Vec<String>,
}

impl<'a> From<&'a String> for Channel {
    fn from(str: &String) -> Channel {
        Channel { parts: str.split('.').map(String::from).collect() }
    }
}
impl<'a> From<&'a str> for Channel {
    fn from(str: &str) -> Channel {
        Channel { parts: str.split('.').map(String::from).collect() }
    }
}
impl From<String> for Channel {
    fn from(str: String) -> Channel {
        Channel { parts: str.split('.').map(String::from).collect() }
    }
}

impl ToString for Channel {
    fn to_string(&self) -> String {
        self.parts.join(".")
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
            if self.parts[i] != other.parts[i] {
                return false;
            }
        }
        // if the last token was a wildcard, accept any parts after it
        last_was_wildcard
    }
}
