use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf>{
    data: HashMap<&'buf str, Value<'buf>>
}

impl<'buf> QueryString<'buf> {
    fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    //a=1&b=2&c&d=3&d=4
    fn from(query_string: &'buf str) -> Self {
        let mut data = HashMap::new();
        
        for sub_str in query_string.split('&') {
            if let Some(i) = sub_str.find('=') {
                let key = &sub_str[..i];
                let value = &sub_str[i+1..];
                /*
                - if key is not in hashmap, insert single
                - if key is in hashmap,
                    - if single, make multiple,
                    - if multiple, append
                */
                data.entry(key).and_modify(|existing| match existing {
                    Value::Single(prev) => {
                        *existing = Value::Multiple(vec![prev, value]);
                    },
                    Value::Multiple(vec) => {
                        vec.push(value);
                    }
                }).or_insert(Value::Single(value));
            }
        }

        QueryString{data}

    }
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}