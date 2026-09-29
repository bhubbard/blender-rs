use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    let iter = map.iter();
    for (key, value) in iter {
        println!("{:?} -> {:?}", key, value);
    }
}
