use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc

pub struct QueryString<'a> {
    data: HashMap<&'a str, Value<'a>>,
}

impl<'a> QueryString<'a> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'a> From<&'a str> for QueryString<'a> {
    fn from(s: &'a str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split("&") {
            
        }
        QueryString { data }
    }
}

pub enum Value<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}
