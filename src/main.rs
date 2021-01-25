use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::str::FromStr;

fn main() {
    let matches = App::new("gene-conf (gc)")
        .version("0.0.1")
        .about("Joona Piirainen <joona.piirainen@gmail.com>")
        .about("Auto generate boring config boilerplate")
        .arg(
            Arg::from("<type of config> 'The type to use'")
                .possible_values(&["prettier", "p", "typescript", "ts"])
                .required(true),
        )
        .arg(
            Arg::new("OPTIONS")
                .about("The options for chosen config type.")
                .short('o')
                .takes_value(true),
        )
        .get_matches();

    let type_of_config = matches
        .value_of_t("type of config")
        .unwrap_or_else(|e| e.exit());

    match type_of_config {
        Config::Prettier => println!("found Prettier"),
        Config::TypeScript => println!("found TypeScript"),
    }

    let options = matches.value_of("OPTIONS").unwrap_or("default");

    println!("options: {}", options);

    let foo = read_prettier(String::from("testPrettier.json")).unwrap();
}

enum Config {
    Prettier,
    TypeScript,
}

impl FromStr for Config {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "prettier" => Ok(Config::Prettier),
            "p" => Ok(Config::Prettier),
            "typescript" => Ok(Config::TypeScript),
            "ts" => Ok(Config::TypeScript),
            _ => Err("no match"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct PrettierConf {
    foo: String,
}

fn read_prettier(path: String) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let file = File::open(path).expect("file should open read only");

    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");

    println!("{}", json.get("foo").expect("foo"));

    Ok(())
}
