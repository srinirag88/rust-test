use std::collections::HashMap;

pub fn vector() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{}", v[0]);
}

pub fn vector_iterate() {
    let v = vec![1, 2, 3];
    for elem in &v {
        println!("{}", elem);
    }
}

pub fn hash_map() {
    let mut key_val = HashMap::new();
    key_val.insert("test", "mine");
    println!("{}", key_val.get("test").unwrap());
}
