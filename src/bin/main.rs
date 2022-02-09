use clap::{app_from_crate, arg, App, AppSettings};
use toml_edit::{value, Document};
use versionman::{ops, VersionIncrement};

fn main() {
    let matches = app_from_crate!()
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("increment")
                .about("Increment cargo project version")
                .arg(
                    arg!(<VERSION>)
                        .help("Version to increment")
                        .possible_values(VersionIncrement::possible_values()),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("increment", sub_matches)) => {
            let file_content = std::fs::read_to_string("./Cargo.toml").unwrap();
            let mut toml_content = file_content.parse::<Document>().unwrap();

            let old_version = toml_content["package"]["version"].as_str().unwrap();
            let new_version = ops::increment_version_string(
                old_version.to_string(),
                sub_matches
                    .value_of_t::<VersionIncrement>("VERSION")
                    .unwrap(),
            );

            let formatted_version = ops::format_version(&new_version);
            toml_content["package"]["version"] = value(formatted_version);
            std::fs::write("./Cargo.toml", toml_content.to_string().as_bytes()).unwrap();
        }
        _ => unreachable!("SubcommandRequiredElseHelp prevents `None`"),
    };
}
