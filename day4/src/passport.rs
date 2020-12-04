use regex::Regex;
use std::collections::HashMap;

pub struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    pub fn new(string: &str, regex: &Regex) -> Passport {
        let mut fields = HashMap::new();
        regex.captures_iter(&string).for_each(|cap| {
            fields.insert(String::from(&cap[1]), String::from(&cap[2]));
        });

        Passport { fields: fields }
    }

    pub fn fields(&self) -> &HashMap<String, String> {
        &self.fields
    }
}
