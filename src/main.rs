extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;
use std::fs::File;
use std::io::prelude::*;

// json other https://github.com/serde-rs/json-benchmark/blob/master/Cargo.toml
// ? operator https://m4rw3r.github.io/rust-questionmark-operator

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

fn print_an_address() -> Result<(), Error> {
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    // parse
    let addr: Address = serde_json::from_str(&j)?;
    println!("{}", addr.city);

    Ok(())
}

#[derive(Deserialize)]
struct Config {
    port: u32,
}

impl Config {
    fn new() -> Config {
        Config {
            port: 0,
        }
    }
}

//static mut config: Config = Config::new();

fn main() {
    println!("Hello, world!");
    //print_an_address();

    // do {
    //     mut f <- File::create("foo.txt");
    //     f.write_all(b"Hello world!")
    // }

    //let c = Config::new();

    let file_name = "config.json";
    let mut json = String::new();
    let mut file = File::open(file_name).unwrap();
    file.read_to_string(&mut json).unwrap();
    let config: Config = serde_json::from_str(&json).unwrap();

    println!("{}", config.port);
}
