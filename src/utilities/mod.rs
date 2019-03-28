use serde::Deserialize;

pub mod translation;
pub mod convert;
pub mod dependencies;

#[derive(Debug, Deserialize)]
pub enum Utility {
    ConvertProject,
    Translation,
    Dependencies
}

pub fn find(utility: &str) -> Option<Utility> {
    return match utility {
        "convertproject" => Some(Utility::ConvertProject),
        "translation" => Some(Utility::Translation),
        "dependencies" => Some(Utility::Dependencies),
        _ => None
    }
}

pub fn run(utility: &Utility) -> Result<(), std::io::Error> {
    #[allow(unreachable_patterns)]
    return match utility {
        Utility::ConvertProject => convert::run(),
        Utility::Translation => translation::check(),
        Utility::Dependencies => {
            // TODO: think about what we want to return here - can we pass args to utilities?
            return dependencies::show();
        }
        _ => Err(error!("Utility not implemented"))
    }
}
