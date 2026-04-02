use std::collections::HashMap;

use nano_templater::templater::Templater;

fn main() {
    let template = "Hello, {name}!";
    let templater = Templater::prepare(&template);
    let mut map = HashMap::new();
    map.insert("name".to_string(), "Foxie");
    let hello_foxie = templater.format(&map).unwrap();
    map.insert("name".to_string(), "World");
    let hello_world = templater.format(&map).unwrap();
    println!("{}\n{}", hello_foxie, hello_world);
}
