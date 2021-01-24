use std::str::FromStr;

use clap::{App, Arg};

enum Config {
    Prettier,
    TypeScript,
}

impl FromStr for Config {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Prettier" => Ok(Config::Prettier),
            "p" => Ok(Config::Prettier),
            "TypeScript" => Ok(Config::TypeScript),
            "ts" => Ok(Config::TypeScript),
            _ => Err("no match")
        }
    }
}

fn main() {
    let m = App::new("gene-conf (gc)")
        .arg(
            Arg::from("<type of config> 'The type to use'")
            .possible_values(&["Prettier", "p", "TypeScript", "ts"]),
        )
        .get_matches();

    let t = m.value_of_t("type of config").unwrap_or_else(|e| e.exit());

    match t {
       Config::Prettier => println!("found Prettier"),
       Config::TypeScript => println!("found TypeScript"),
    }
}
