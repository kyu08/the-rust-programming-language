use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    let k = String::from("k");
    let v = String::from("valueeeeeeeeeee");
    scores.insert(&k, v);
    let v = scores.get(&k);
    match v {
        Some(v) => println!("{}", v),
        None => println!("none"),
    }
}
