use crate::{HEMTTError, Project};

pub fn run(p: Project) -> Result<(), HEMTTError> {
    let items = ["CBA", "ACE", "Vanilla"];
    let selection = {
        let mut select = dialoguer::Select::new();
        select.default(0);
        select.items(&items);
        select.interact_opt()?
    };
    if selection.is_none() {
        unimplemented!()
    }
    let selection = items[selection.unwrap()];
    println!("Template: {}", selection);
    // clone template
    match p.template.as_ref() {
        "" => {}
        _ => {
            let repo = if p.template.starts_with("http") {
                p.template
            } else {
                format!("https://github.com/hemtt/{}", p.template)
            };
            match git2::Repository::clone(&repo, "./.hemtt/template") {
                Ok(_) => println!("Template Cloned"),
                Err(e) => panic!("Failed to clone: {}", e),
            };
        }
    }
    Ok(())
}
