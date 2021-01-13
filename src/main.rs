mod file;
mod models;

use anyhow::{bail, Result};
use clap::{crate_version, load_yaml, App};
use log::{debug, info, warn};
use models::template_objects::TemplateList;
use std::env;
use std::process::exit;

// Assign memory space for auditable list of crates used
static COMPRESSED_DEPENDENCY_LIST: &[u8] = auditable::inject_dependency_list!();

fn main() -> Result<()> {
    if let Err(e) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "warn");
    }
    env_logger::try_init()?;

    let filename: &str;
    let clap_yaml = load_yaml!("cli.yaml");
    let matches = App::from(clap_yaml).version(crate_version!()).get_matches();

    if matches.is_present("audit") {
        println!(
            "This binary makes use of the rust-audit crate for validating versions of crates used."
        );
        println!("To validate crate versions with this binary, check out the auditable-extract crate for details.");
        println!(
            "Auditable dependencies recorded in this binary: {}",
            COMPRESSED_DEPENDENCY_LIST[0]
        );
        exit(0);
    }

    if let Some(f) = matches.value_of("templatefile") {
        filename = f;
    } else {
        filename = "/usr/share/remarkable/templates/templates.json";
    }

    if let Ok(templates) = TemplateList::new(filename) {
        match matches.subcommand_name() {
            Some("add") => add_entry(templates),
            Some("remove") => remove_entry(templates),
            None => bail!("No subcommand specified."),
            _ => bail!("Invalid option provided to subcommand."),
        }
    } else {
        bail!("templates.json file doesn't exist or is unreadable.");
    }
    Ok(())
}

fn add_entry(tl: TemplateList) {}
fn remove_entry(tl: TemplateList) {}
