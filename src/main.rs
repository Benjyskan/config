use dialoguer::Select;
use std::process::Command;

use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::path::PathBuf;

// Yaml structs

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    configs: Vec<Info>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    name: String,
    files: Vec<String>,
}

fn main() {
    // parse yaml config file.
    let config = parse_yml();
    // collect all options in config file.
    let options: Vec<String> = config
        .configs
        .iter()
        .map(|info| info.name.clone())
        .collect();

    // prompt `Select`
    let selection = Select::new()
        .items(&options)
        .default(0)
        .interact_opt()
        .unwrap();
    match selection {
        Some(index) => {
            // Edit selected files
            edit(index, &config);
        }
        None => {
            println!("Goodbye!");
        }
    }
}

fn parse_yml() -> Config {
    let home: PathBuf = home_dir().expect("Failed to get home directory");
    // let file_path = PathBuf::from(home_dir).join("my_file.txt");
    // let file = std::fs::File::open(file_path)?;

    #[cfg(debug_assertions)]
    let config_path: &str = "configs.yml";

    #[cfg(not(debug_assertions))]
    // let config_path: &str = "~/bin/configs.yml";
    let config_path: PathBuf = PathBuf::from(home).join("bin").join("configs.yml");

    println!("parsing {:?}", config_path);

    // Get file
    let f = std::fs::File::open(config_path).expect("Could not open file.");
    // Get Config
    let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    // println!("Parsed config:\n{:?}", scrape_config);
    scrape_config
}

fn edit(option_index: usize, config: &Config) {
    let files = &config.configs[option_index].files;
    Command::new("zsh")
        .arg("-c")
        .arg(format!("$EDITOR {}", files.join(" ")))
        .status()
        .expect("Woopsi!");
}
