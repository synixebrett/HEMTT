use armake2::preprocess::*;
use armake2::io::{Input,Output}; // I definitely shouldn't be using this
// use pbr::ProgressBar;
use petgraph::Graph;
use petgraph::dot::{Dot,Config};
use rayon::prelude::*;
use rayon::iter::ParallelBridge;
use regex::Regex;
use walkdir::WalkDir;

use std::fs::File;
// use std::io::BufReader;
use std::io::{stdout,Read,Write};
use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::error::*;
use crate::project::*;


// Rust does not make it nice to move code out into functions

// type lockedgraph = std::sync::MutexGuard<Graph<String,()>>;
// type lockedhash = std::collections::HashMap<std::string::String, petgraph::graph::NodeIndex>;
// fn add_dep_edge(deps: lockedgraph, name2node: lockedhash, name:String, req: String) -> (){
//     // if !name2node.lock().unwrap().contains_key(&req){
//     //     let nodenum = deps.lock().unwrap().add_node(req.to_string());
//     //     name2node.lock().unwrap().insert(req.to_string(),nodenum);
//     // }
//     // let requirementind = name2node.lock().unwrap().get(&req).unwrap().clone();
//     // let dependentind = name2node.lock().unwrap().get(&name).unwrap().clone();
//     // deps.lock().unwrap().add_edge(requirementind,dependentind,());
// }

pub fn show() -> Result<(), std::io::Error> {
    let mut p = crate::project::get_project().unwrap();
    let configs: Vec<PathBuf> = WalkDir::new("addons").into_iter().map(|e| e.unwrap_or_print().path().to_path_buf()).filter(|e| e.ends_with("config.cpp")).collect();
    p.include.push(PathBuf::from("."));

    let gimmeCfgPatch = Regex::new(r"(?i)class CfgPatches").unwrap();
    let gimmeClass = Regex::new(r"(?i)class ([^{]*)").unwrap();
    let gimmeReqs = Regex::new(r"(?i)requiredAddons.*\{(.*)\}").unwrap();
    let reqsStart = Regex::new(r"(?i)requiredAddons.*\{").unwrap();
    let reqsEnd = Regex::new(r"\};").unwrap();
    let gimmeDeps = Regex::new("\"(.*?)\"").unwrap();

    // This is slow enough that we probably need a progress bar :(
    let mut deps = Arc::new(Mutex::new(Graph::<_, ()>::new()));
    let mut name2node = Arc::new(Mutex::new(HashMap::new()));
    let vecconf = configs.par_iter().map(|config| {
        // turn this into a .map eventually
        // this obviously shouldn't output to stdout
        let mut buffer = String::new();
        File::open(config).unwrap().read_to_string(&mut buffer).expect("Failed to read config"); // TODO: improve error
        // Magic number here accesses the actual string rather than "preprocess info"
        // Would be good to turn this into a lazily-evaluated iterator from which we can consume lines
        let processed = armake2::preprocess::preprocess(buffer, Some(config.to_path_buf()),&p.include).unwrap().0;
        let processed_lines = processed.split('\n');
        let mut looking_for = "patches";
        let mut looking_for_deps = false;
        let mut name = "";
        for line in processed_lines {
            if (looking_for == "reqs") {
                if (gimmeReqs.is_match(line)) {
                    // Zeroth match corresponds to whole line - first match is first capture group
                    // Todo - move this out into a function
                    gimmeDeps.captures_iter(line).map(|s| s.get(1).unwrap().as_str()).for_each(|req| {
                        if !name2node.lock().unwrap().contains_key(req){
                            let nodenum = deps.lock().unwrap().add_node(req.to_string());
                            name2node.lock().unwrap().insert(req.to_string(),nodenum);
                        }
                        let requirementind = name2node.lock().unwrap().get(req).unwrap().clone();
                        let dependentind = name2node.lock().unwrap().get(name).unwrap().clone();
                        deps.lock().unwrap().add_edge(requirementind,dependentind,());
                    });
                    looking_for = "patches";
                    looking_for_deps = false;
                    break;
                } else if reqsStart.is_match(line) {
                    gimmeDeps.captures_iter(line).map(|s| s.get(1).unwrap().as_str()).for_each(|req| {
                        if !name2node.lock().unwrap().contains_key(req){
                            let nodenum = deps.lock().unwrap().add_node(req.to_string());
                            name2node.lock().unwrap().insert(req.to_string(),nodenum);
                        }
                        let requirementind = name2node.lock().unwrap().get(req).unwrap().clone();
                        let dependentind = name2node.lock().unwrap().get(name).unwrap().clone();
                        deps.lock().unwrap().add_edge(requirementind,dependentind,());
                    });
                    looking_for_deps = true;
                    continue;
                } else if looking_for_deps {
                    if reqsEnd.is_match(line) {
                        gimmeDeps.captures_iter(line).map(|s| s.get(1).unwrap().as_str()).for_each(|req| {
                            if !name2node.lock().unwrap().contains_key(req){
                                let nodenum = deps.lock().unwrap().add_node(req.to_string());
                                name2node.lock().unwrap().insert(req.to_string(),nodenum);
                            }
                            let requirementind = name2node.lock().unwrap().get(req).unwrap().clone();
                            let dependentind = name2node.lock().unwrap().get(name).unwrap().clone();
                            deps.lock().unwrap().add_edge(requirementind,dependentind,());
                        });
                        looking_for = "patches";
                        looking_for_deps = false;
                        break;
                    } else { // hope no-one "makes a comment like this"
                        gimmeDeps.captures_iter(line).map(|s| s.get(1).unwrap().as_str()).for_each(|req| {
                            if !name2node.lock().unwrap().contains_key(req){
                                let nodenum = deps.lock().unwrap().add_node(req.to_string());
                                name2node.lock().unwrap().insert(req.to_string(),nodenum);
                            }
                            let requirementind = name2node.lock().unwrap().get(req).unwrap().clone();
                            let dependentind = name2node.lock().unwrap().get(name).unwrap().clone();
                            deps.lock().unwrap().add_edge(requirementind,dependentind,());
                        });
                        continue;
                    }
                }
            }
            if ((looking_for == "class") && gimmeClass.is_match(line)) {
                // Zeroth match corresponds to whole line - first match is first capture group
                name = gimmeClass.captures(line).unwrap().get(1).unwrap().as_str().trim();
                if !name2node.lock().unwrap().contains_key(name){
                    let nodenum = deps.lock().unwrap().add_node(name.to_string());
                    name2node.lock().unwrap().insert(name.to_string(),nodenum);
                }
                looking_for = "reqs"; // TODO - handle reqs not existing
            }
            if gimmeCfgPatch.is_match(line) {
                looking_for = "class";
            }
        }
        return processed;


        // really, we want to be able to tell preprocess to stop once it 
        // gets to a line matching certain criteria
        // i.e, we want to parse up to requiredAddons[] and no further
        // https://github.com/KoffeinFlummi/armake2/blob/40fabd915514ffda372ec012b35ed4190d0e0515/src/preprocess.rs#L369
        // but that's just perf so let's leave it for now
    }).collect(); // The collect makes this blocking
    // This prints first? I guess we need to wait for the loop to finish or something weird?
    let graph = deps.lock().unwrap();
    println!("{:?}", Dot::with_config(&*graph, &[Config::EdgeNoLabel]));
    let mut file = File::create("test.txt")?;
    let confs: Vec<String> = vecconf;
    file.write(confs.join("\n").as_bytes());
    Ok(())
}
