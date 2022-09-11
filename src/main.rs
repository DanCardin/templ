use std::collections::HashMap;

use templ::format;

fn main() {
    let mut foo = HashMap::new();
    foo.insert("test".to_string(), "meow");

    let result = format("wat {test} wat", &foo).unwrap();
    println!("{}", result);
}
