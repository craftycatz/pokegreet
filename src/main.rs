use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
struct Pokemon {
    name: String,
    sprites: Sprites,
}

#[derive(Debug, Deserialize)]
struct Sprites {
    front_default: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("./README.md")?;
    let pokemon_id = rand::thread_rng().gen_range(1..=1000);
    let res = reqwest::blocking::get(&format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_id))?;
    let result: Pokemon = res.json()?;
    writeln!(
        &mut file,
        r#"<p align="center">
            <img src="{}" width="150" height="150">
          </p>
          <h3 align="center">You have been greeted by - <b>{}</b></h3>
          <h3 align="center">Have a nice day!</h3>"#,
        result.sprites.front_default,
        result.name.to_uppercase()
    )?;
    Ok(())
}