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
