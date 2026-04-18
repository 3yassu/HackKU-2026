/*
x = neither enemies nor foxes can spawn there
f = fox placeable here
s = enemy spawn
u/r/l/d: enemy moves up/right/left/down from here
k = enemy despawn
this is really useful code.
*/
use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Map {
    hitboxes: Vec<Vec<String>>,
    tileset: String,
    rounds: u8,
}

fn load_map(f: String) {
    let contents = fs::read_to_string("f").unwrap();
    let data: Map = serde_json::from_str(&contents).unwrap();
    println!("{:?}", data);
}
