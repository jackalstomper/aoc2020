use super::passport::Passport;
use regex::Regex;
use std::collections::HashMap;

type ValidationMap = HashMap<&'static str, Validation>;

pub struct Validator {
    validations: ValidationMap,
}

impl Validator {
    pub fn new() -> Validator {
        Validator {
            validations: validations(),
        }
    }

    pub fn part1_validate(&self, passport: &Passport) -> bool {
        for &field_name in self.validations.keys() {
            if passport.fields().get(field_name).is_none() {
                return false;
            }
        }

        true
    }

    pub fn part2_validate(&self, passport: &Passport) -> bool {
        for &field_name in self.validations.keys() {
            let val = match passport.fields().get(field_name) {
                None => return false,
                Some(e) => e,
            };

            match self.validations.get(field_name) {
                Some(v) => {
                    if !v.validate(val) {
                        return false;
                    }
                }
                None => println!("No validator found for {}!", field_name),
            }
        }

        true
    }
}

struct Validation {
    regex: Regex,
    validate_cb: Box<dyn Fn(&regex::Captures) -> Result<bool, &'static str>>,
}

impl Validation {
    fn validate(&self, target: &str) -> bool {
        let cap = match self.regex.captures(target) {
            None => return false,
            Some(e) => e,
        };

        match (self.validate_cb)(&cap) {
            Err(msg) => {
                println!("{}", msg);
                return false;
            }
            Ok(b) => b,
        }
    }
}

fn validations() -> ValidationMap {
    let mut validators = HashMap::new();
    validators.insert(
        "byr",
        Validation {
            regex: Regex::new(r"^\d\d\d\d$").unwrap(),
            validate_cb: Box::new(|cap| {
                let n: i32 = match cap[0].parse() {
                    Err(_) => return Err("Failed to parse byr value"),
                    Ok(n) => n,
                };

                Ok(n >= 1920 && n <= 2002)
            }),
        },
    );

    validators.insert(
        "iyr",
        Validation {
            regex: Regex::new(r"^\d\d\d\d$").unwrap(),
            validate_cb: Box::new(|cap| {
                let n: i32 = match cap[0].parse() {
                    Err(_) => return Err("Failed to parse iyr value"),
                    Ok(n) => n,
                };

                Ok(n >= 2010 && n <= 2020)
            }),
        },
    );

    validators.insert(
        "eyr",
        Validation {
            regex: Regex::new(r"^\d\d\d\d$").unwrap(),
            validate_cb: Box::new(|cap| {
                let n: i32 = match cap[0].parse() {
                    Err(_) => return Err("Failed to parse eyr value"),
                    Ok(n) => n,
                };

                Ok(n >= 2020 && n <= 2030)
            }),
        },
    );

    validators.insert(
        "hgt",
        Validation {
            regex: Regex::new(r"^(\d+)(cm|in)$").unwrap(),
            validate_cb: Box::new(|cap| {
                let n: i32 = match cap[1].parse() {
                    Err(_) => return Err("Failed to parse hgt value"),
                    Ok(n) => n,
                };

                if &cap[2] == "cm" {
                    Ok(n >= 150 && n <= 193)
                } else {
                    Ok(n >= 59 && n <= 76)
                }
            }),
        },
    );

    validators.insert(
        "hcl",
        Validation {
            regex: Regex::new(r"^#[0-9a-f]{6}$").unwrap(),
            validate_cb: Box::new(|_| Ok(true)),
        },
    );

    validators.insert(
        "ecl",
        Validation {
            regex: Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap(),
            validate_cb: Box::new(|_| Ok(true)),
        },
    );

    validators.insert(
        "pid",
        Validation {
            regex: Regex::new(r"^\d{9}$").unwrap(),
            validate_cb: Box::new(|_| Ok(true)),
        },
    );

    validators
}
