mod file;
mod models;

use crate::file::{is_template_file, write_file};
use crate::models::template_objects::TemplateObject;
use anyhow::{bail, Result};
use clap::{crate_version, load_yaml, App, ArgMatches};
use log::{debug, info, warn};
use models::template_objects::TemplateList;
use regex::Regex;
use serde_json::Value;
use std::env;
use std::path::Path;
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

    if let Some(f) = matches.value_of("templatefile") {
        filename = f;
    } else {
        filename = "/usr/share/remarkable/templates/templates.json";
    }

    let mut templates: TemplateList = TemplateList::new(filename)?;
    match matches.subcommand_name() {
        Some("add") => {
            add_entry(&mut templates, matches.clone())?;
            write_file(filename, serde_json::to_string_pretty(&templates)?)?;
            Ok(())
        }
        Some("remove") => {
            remove_entry(&mut templates, matches.clone())?;
            write_file(filename, serde_json::to_string_pretty(&templates)?)?;
            Ok(())
        }
        None => bail!("No subcommand specified."),
        _ => bail!("Invalid option provided to subcommand."),
    }
}

fn add_entry(tl: &mut TemplateList, matches: ArgMatches) -> Result<()> {
    let command_matches: &ArgMatches = matches
        .subcommand_matches("add")
        .expect("no arguments specified.");

    let mut template_object = TemplateObject::default();

    template_object.name = command_matches.value_of("name").unwrap().to_string();
    template_object.filename = command_matches.value_of("filename").unwrap().to_string();

    let png_regex = Regex::new(r"(.*)\.png$").unwrap();
    let svg_regex = Regex::new(r"(.*)\.svg$").unwrap();

    if png_regex.is_match(template_object.filename.clone().as_str()) {
        if is_template_file(template_object.filename.clone()) {
            let caps = png_regex
                .captures(template_object.filename.as_str())
                .unwrap();
            template_object.filename = caps.get(1).unwrap().as_str().to_string();
        } else {
            bail!("Specified png file doesn't exist in /usr/share/remarkable/templates.")
        }
    } else if svg_regex.is_match(template_object.filename.clone().as_str()) {
        if is_template_file(template_object.filename.clone()) {
            let caps = svg_regex
                .captures(template_object.filename.as_str())
                .unwrap();
            template_object.filename = caps.get(1).unwrap().as_str().to_string();
        } else {
            bail!("Specified png file doesn't exist in /usr/share/remarkable/templates.")
        }
    } else {
        // filename wasn't either a png or svg, so we'll check to make sure png exists at least.

        if is_template_file(format!("{}.png", template_object.filename.clone())) == false {
            bail!("Specified template file doesn't exist in png format on device.")
        }
    }

    let is_landscape: bool = command_matches.is_present("landscape");
    if is_landscape {
        template_object.landscape = Some(Value::from(true));
        template_object.icon_code = String::from("\\ue9fd");
    } else {
        template_object.landscape = Some(Value::from(false));
        template_object.icon_code = String::from("\\ue9fe");
    };

    if let Some(icon) = command_matches.value_of("iconcode") {
        template_object.icon_code = icon.to_string();
    };

    if let categories = command_matches.values_of_t::<String>("categories").unwrap() {
        template_object.categories = categories;
    };

    debug!("{:#?}", serde_json::to_string(&template_object)?);
    tl.templates.retain(|obj| obj.name != template_object.name);
    tl.templates.push(template_object);

    Ok(())
}
fn remove_entry(tl: &mut TemplateList, matches: ArgMatches) -> Result<()> {
    let command_matches: &ArgMatches = matches.subcommand_matches("remove").unwrap();

    let template_name = command_matches.value_of("name").unwrap().to_string();
    tl.templates.retain(|obj| obj.name != template_name);
    Ok(())
}
