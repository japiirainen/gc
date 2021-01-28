#![allow(non_snake_case)]
use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::fs;
use std::str::FromStr;

fn main() {
    let matches = App::new("gen-conf (gc)")
        .version("0.0.1")
        .about("Joona Piirainen <joona.piirainen@gmail.com>")
        .about("Auto generate boring config boilerplate")
        .arg(
            Arg::from("<type of config> 'The type to use'")
                .possible_values(&["prettier", "p", "typescript", "ts"])
                .required(true),
        )
        .arg(
            Arg::new("pw")
                .about("what print width to use")
                .short('p')
                .takes_value(true)
                .default_value("100")
                .required(false),
        )
        .arg(
            Arg::new("sq")
                .about("if you provide this flag prettier configuration uses single quotes")
                .short('s')
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::new("ut")
                .about("if you provide this flag prettier configuration uses tabs")
                .short('u')
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::new("tw")
                .about("what tab width to use")
                .short('t')
                .takes_value(true)
                .default_value("3")
                .required(false),
        )
        .arg(
            Arg::new("semi")
                .about("if you provide this flag prettier configuration uses semi colons")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::new("arrowParens")
                .about("what print width to use")
                .short('a')
                .takes_value(true)
                .possible_values(&["avoid", "always"])
                .default_value("avoid")
                .required(false),
        )
        .arg(
            Arg::new("react")
                .about("if you provide this flag the tsconfig will be valid for react")
                .takes_value(false)
                .short('r')
                .required(false),
        )
        .arg(
            Arg::new("node")
                .about("if you provide this flag the tsconfig will be valid for node")
                .takes_value(false)
                .short('n')
                .required(false),
        )
        .get_matches();

    let type_of_config = matches
        .value_of_t("type of config")
        .unwrap_or_else(|e| e.exit());

    match type_of_config {
        Config::Prettier => {
            let pw = matches.value_of("pw").unwrap();
            let mut sq = false;
            let mut ut = false;
            let tw = matches.value_of("tw").unwrap();
            let mut semi = false;
            let ap = matches.value_of("arrowParens").unwrap();
            if matches.is_present("sq") {
                sq = true
            }
            if matches.is_present("ut") {
                ut = true
            }
            if matches.is_present("semi") {
                semi = true
            }
            let pconf = PrettierConf::new(
                pw.parse::<u8>().unwrap(),
                sq,
                ut,
                tw.parse::<u8>().unwrap(),
                semi,
                ap.to_string(),
            );
            let pc = serde_json::to_string(&pconf).unwrap();
            fs::write(".prettierrc.json", pc).unwrap();
        }
        Config::TypeScript => {
            if matches.is_present("node") {
                let tsc = TsConfigNode::new(
                    "dist".to_string(),
                    "es5".to_string(),
                    "commonjs".to_string(),
                    true,
                    true,
                    true,
                    true,
                );

                let conf = TsConfN {
                    compilerOptions: tsc,
                    exclude: vec!["node_modules".to_string()],
                };
                let tc = serde_json::to_string(&conf).unwrap();

                fs::write("tsconfig.json", tc).unwrap();
            }
            if matches.is_present("react") {
                let tsc = TsConfigReact::new(
                    "dist".to_string(),
                    "es5".to_string(),
                    vec![
                        "dom".to_string(),
                        "dom.iterable".to_string(),
                        "esnext".to_string(),
                    ],
                    "commonjs".to_string(),
                    true,
                    true,
                    true,
                    true,
                    "react-jsx".to_string(),
                );
                let conf = TsConfR {
                    compilerOptions: tsc,
                    exclude: vec!["node_modules".to_string()],
                };
                let tc = serde_json::to_string(&conf).unwrap();
                fs::write("tsconfig.json", tc).unwrap();
            }
        }
    }
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
    printWidth: u8,
    singleQuote: bool,
    useTabs: bool,
    trailingComma: String,
    tabWidth: u8,
    semi: bool,
    arrowParens: String,
}

impl PrettierConf {
    fn new(
        printWidth: u8,
        singleQuote: bool,
        useTabs: bool,
        tabWidth: u8,
        semi: bool,
        arrowParens: String,
    ) -> Self {
        PrettierConf {
            printWidth,
            singleQuote,
            useTabs,
            trailingComma: String::from("es5"),
            tabWidth,
            semi,
            arrowParens,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TsConfN {
    compilerOptions: TsConfigNode,
    exclude: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TsConfR {
    compilerOptions: TsConfigReact,
    exclude: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TsConfigNode {
    outDir: String,
    target: String,
    module: String,
    strict: bool,
    esModuleInterop: bool,
    skipLibCheck: bool,
    forceConsistentCasingInFIleNames: bool,
}

impl TsConfigNode {
    fn new(
        outDir: String,
        target: String,
        module: String,
        strict: bool,
        esModuleInterop: bool,
        skipLibCheck: bool,
        forceConsistentCasingInFIleNames: bool,
    ) -> Self {
        TsConfigNode {
            outDir,
            target,
            module,
            strict,
            esModuleInterop,
            skipLibCheck,
            forceConsistentCasingInFIleNames,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TsConfigReact {
    outDir: String,
    target: String,
    lib: Vec<String>,
    module: String,
    strict: bool,
    esModuleInterop: bool,
    skipLibCheck: bool,
    forceConsistentCasingInFIleNames: bool,
    jsx: String,
}

impl TsConfigReact {
    fn new(
        outDir: String,
        target: String,
        lib: Vec<String>,
        module: String,
        strict: bool,
        esModuleInterop: bool,
        skipLibCheck: bool,
        forceConsistentCasingInFIleNames: bool,
        jsx: String,
    ) -> Self {
        TsConfigReact {
            outDir,
            target,
            lib,
            module,
            strict,
            esModuleInterop,
            skipLibCheck,
            forceConsistentCasingInFIleNames,
            jsx,
        }
    }
}
