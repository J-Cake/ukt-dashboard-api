use serde::{Serialize, Deserialize};
use serde_urlencoded;
use serde_qs; // serde_qs doesn't seem to be available in the playground.

#[derive(Serialize, Deserialize)]
pub struct Test {
    arr: Vec<String>
}

fn main() {
    let test = Test {
        arr: vec!["Hello".into(), "World".into()]
    };

    let urlencoded = serde_urlencoded::to_string(&test);
    let qs = serde_qs::to_string(&test);

    println!("`urlencoded` {urlencoded:?}");
    println!("`qs` {qs:?}");
}