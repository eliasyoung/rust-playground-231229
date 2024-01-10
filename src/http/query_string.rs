use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc
#[derive(Debug)]
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
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }
            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                 
                        *existing = Value::Multiple(vec![prev_val,val]);

                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(&val));
        }
        QueryString { data }
    }
}

#[derive(Debug)]
pub enum Value<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}
