/*

foobar&cheese&yeet -> [foobar, cheese, yeet]
foo=bar&a=b&f=d -> {foo: bar, a: b, f: d}
foobar=a&foobar=b => {foobar: [a, b]}

*/

use std::{collections::HashMap, fmt::Display, string::FromUtf8Error};

use urlencoding::{decode, encode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    pub unnamed: Vec<String>,
    pub named: HashMap<String, Vec<String>>,
}

impl Query {
    #[inline]
    pub fn new() -> Self {
        Self {
            unnamed: Vec::new(),
            named: HashMap::new(),
        }
    }

    pub fn parse(query: &str) -> Result<Self, FromUtf8Error> {
        let mut named: HashMap<String, Vec<String>> = HashMap::new();
        let mut unnamed = Vec::new();

        for item in query.split('&') {
            if item.is_empty() {
                continue;
            }

            let mut pair = item.splitn(2, '=');
            let first = decode(pair.next().unwrap())?.into_owned();

            if let Some(second) = pair.next() {
                // Named Value
                let second = decode(second)?.into_owned();
                if let Some(values) = named.get_mut(&first) {
                    values.push(second);
                } else {
                    named.insert(first, vec![second]);
                }
            } else {
                // Unnamed Value
                unnamed.push(first);
            }
        }

        Ok(Self { named, unnamed })
    }

    pub fn get_value(&self, key: &str) -> Option<&str> {
        match self.named.get(key) {
            Some(v) => match v.first() {
                Some(v) => Some(v),
                None => None,
            },
            None => None,
        }
    }

    pub fn get_value_mut(&mut self, key: &str) -> Option<&mut String> {
        match self.named.get_mut(key) {
            Some(v) => match v.first_mut() {
                Some(v) => Some(v),
                None => None,
            },
            None => None,
        }
    }

    #[inline]
    pub fn get_values(&self, key: &str) -> Option<&Vec<String>> {
        self.named.get(key)
    }

    #[inline]
    pub fn get_values_mut(&mut self, key: &str) -> Option<&mut Vec<String>> {
        self.named.get_mut(key)
    }

    #[inline]
    pub fn contains_key(&mut self, key: &str) -> bool {
        self.named.contains_key(key)
    }

    pub fn contains(&self, key: &str) -> bool {
        if self.named.contains_key(key) {
            return true;
        }
        for val in &self.unnamed {
            if val == key {
                return true;
            }
        }
        false
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        match self.unnamed.get(index) {
            Some(v) => Some(v),
            None => None,
        }
    }

    #[inline]
    pub fn first(&self) -> Option<&str> {
        self.get(0)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut String> {
        self.unnamed.get_mut(index)
    }

    pub fn insert(&mut self, key: String, value: String) {
        if let Some(values) = self.named.get_mut(&key) {
            values.push(value);
        } else {
            self.named.insert(key, vec![value]);
        }
    }

    #[inline]
    pub fn remove(&mut self, key: &str) -> Option<Vec<String>> {
        self.named.remove(key)
    }

    pub fn erase(&mut self, value: &str) -> Option<String> {
        let mut index = None;
        for (i, val) in self.unnamed.iter().enumerate() {
            if val == value {
                index = Some(i);
            }
        }
        if let Some(index) = index {
            Some(self.unnamed.remove(index))
        } else {
            None
        }
    }

    pub fn replace(&mut self, key: &str, value: String) {
        if let Some(values) = self.named.get_mut(key) {
            values.clear();
            values.push(value);
        } else {
            self.named.insert(key.into(), vec![value]);
        }
    }

    #[inline]
    pub fn push(&mut self, value: String) {
        self.unnamed.push(value)
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.unnamed.is_empty() {
            for (i, value) in self.unnamed.iter().enumerate() {
                f.write_str(&encode(value))?;
                if i < self.unnamed.len() - 1 {
                    f.write_str("&")?;
                }
            }
            if !self.named.is_empty() {
                f.write_str("&")?;
            }
        }
        if !self.named.is_empty() {
            for (i, (name, values)) in self.named.iter().enumerate() {
                let name = encode(name);
                for (j, value) in values.iter().enumerate() {
                    let value = encode(value);
                    f.write_fmt(format_args!("{name}={value}"))?;
                    if j < values.len() - 1 {
                        f.write_str("&")?;
                    }
                }
                if i < self.named.len() - 1 {
                    f.write_str("&")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_item() {
        let query = Query::parse("Hello%20World").unwrap();

        assert_eq!(query.first().unwrap(), "Hello World");
    }

    #[test]
    fn test_many_items() {
        let query = Query::parse("Hello&World").unwrap();

        assert_eq!(query.get(0).unwrap(), "Hello");
        assert_eq!(query.get(1).unwrap(), "World");
    }

    #[test]
    fn test_single_named() {
        let query = Query::parse("Hello=World").unwrap();

        assert_eq!(query.get_value("Hello").unwrap(), "World");
    }

    #[test]
    fn test_many_named() {
        let query = Query::parse("Hello=World&cheese=foo").unwrap();

        assert_eq!(query.get_value("Hello").unwrap(), "World");
        assert_eq!(query.get_value("cheese").unwrap(), "foo");
    }

    #[test]
    fn test_many_names() {
        let query = Query::parse("Hello=World&Hello=foo").unwrap();

        assert_eq!(query.get_value("Hello").unwrap(), "World");
        assert_eq!(query.get_values("Hello").unwrap(), &vec!["World", "foo"]);
    }

    #[test]
    fn test_display() {
        let mut query = Query::new();

        query.insert("Hello".into(), "cheese".into());
        query.insert("Hello".into(), "bar".into());
        query.push("Yeet".into());

        assert_eq!(query.to_string(), "Yeet&Hello=cheese&Hello=bar");
    }
}
