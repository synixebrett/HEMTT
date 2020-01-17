use super::Command;
use crate::HEMTTError;

mod init;

pub struct Template {}
impl Command for Template {
    fn register(&self) -> clap::App {
        clap::SubCommand::with_name("template")
            .version(*crate::VERSION)
            .about("Manage the project's template")
            .subcommand(clap::SubCommand::with_name("init").about("Initialize the template"))
            .subcommand(
                clap::SubCommand::with_name("addon").about("Create a new addon").arg(
                    clap::Arg::with_name("name")
                        .help("Name of the addon to create")
                        .required(true),
                ),
            )
            .subcommand(
                clap::SubCommand::with_name("function")
                    .arg(clap::Arg::with_name("addon").help("Addon to add function to").required(true))
                    .arg(clap::Arg::with_name("name").help("Name of the function").required(true)),
            )
    }

    fn run(&self, a: &clap::ArgMatches, p: crate::project::Project) -> Result<(), HEMTTError> {
        if crate::project::single_file() {
            return Err(HEMTTError::MultiFileProjectRequired);
        }
        if p.template.is_empty() && a.subcommand_name().is_some() && a.subcommand_name().unwrap() != "init" {
            return Err(HEMTTError::NoTemplate);
        }
        match a.subcommand() {
            ("init", _) => {
                init::run(p)?;
            }
            _ => println!("Not implemented"),
        }
        Ok(())
    }
}
