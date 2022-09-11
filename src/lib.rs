use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::hash::Hash;
use std::str::FromStr;
use std::string::String;

mod fmtstr;
mod formatter;
mod types;

pub use fmtstr::strfmt_map;
pub use formatter::Formatter;
pub use types::{Alignment, FmtError, Result, Sign};

/// Rust-style format a string given a `HashMap` of the variables.
pub fn format<K, T: fmt::Display>(fmtstr: &str, vars: &HashMap<K, T>) -> Result<String>
where
    K: Hash + Eq + FromStr,
{
    let formatter = |mut fmt: Formatter| {
        let k: K = match fmt.key.parse() {
            Ok(k) => k,
            Err(_) => {
                return Err(new_key_error(fmt.key));
            }
        };
        let v = match vars.get(&k) {
            Some(v) => v,
            None => {
                return Err(new_key_error(fmt.key));
            }
        };
        fmt.str(v.to_string().as_str())
    };
    strfmt_map(fmtstr, &formatter)
}

pub trait Format {
    fn format<K, D: fmt::Display>(&self, vars: &HashMap<K, D>) -> Result<String>
    where
        K: Hash + Eq + FromStr;
}

impl Format for String {
    fn format<'a, K, D: fmt::Display>(&self, vars: &HashMap<K, D>) -> Result<String>
    where
        K: Hash + Eq + FromStr,
    {
        format(self.as_str(), vars)
    }
}

impl Format for str {
    fn format<K, D: fmt::Display>(&self, vars: &HashMap<K, D>) -> Result<String>
    where
        K: Hash + Eq + FromStr,
    {
        format(self, vars)
    }
}

fn new_key_error(key: &str) -> FmtError {
    let mut msg = String::new();
    write!(msg, "Invalid key: {}", key).unwrap();
    FmtError::KeyError(msg)
}
