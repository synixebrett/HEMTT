use std::io::Write;

use crate::{Command, HEMTTError, Project};

pub struct Init {}

impl Command for Init {
    fn register(&self) -> clap::App {
        clap::SubCommand::with_name("init")
            .version(*crate::VERSION)
            .about("Initialize a HEMTT Project")
            .arg(clap::Arg::with_name("single_file").long("single-file").takes_value(false))
    }

    fn require_project(&self) -> bool {
        false
    }

    fn run_no_project(&self, args: &clap::ArgMatches) -> Result<(), HEMTTError> {
        let name = ask!("Project Name >");
        let prefix = ask!("Prefix >");
        let author = ask!("Author >");

        // Create settings file in TOML
        let mut out = if args.is_present("single_file") {
            create_file!("./hemtt.toml")?
        } else {
            create_dir!("./.hemtt/")?;
            create_file!("./.hemtt/base.toml")?
        };
        let project = Project::new(name, prefix, author);
        out.write_fmt(format_args!("{}", toml::to_string(&project)?))?;
        Ok(())
    }
}
