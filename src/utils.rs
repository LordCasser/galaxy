
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::error::Error;
pub fn read_file(path: &Path) -> String {
    // let c = std::fs::read_to_string(path).unwrap();
    println!("{}",path.display());
    let p = |err:&dyn Error|{
        eprintln!("Error: {}", err);
        return "".to_string();
    };
    match fs::read_to_string(path) {
        Ok(value) => return value.to_string(),
        Err(err) => p(&err),
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Rule {
    pub issue: String,
    pub discription: String,
    pub patterns: Vec<String>,
}

pub fn prase_json(data: &str) -> Rule {
    let rule: Rule = serde_json::from_str(data).unwrap();
    println!("{}",rule.issue);
    return rule;
}

fn traverse_directories(dir_path: &Path, last_result: &mut Vec<PathBuf>) -> Vec<PathBuf> {
    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path).expect("[X] read_dir call failed") {
            let entry = entry.expect("[X] unable to get entry");
            let path = entry.path();

            if path.is_dir() {
                last_result.push(path.clone());
                traverse_directories(&path, last_result);
            }
        }
    }
    last_result.clone()
}

//find all rules folder by folder
pub fn rule_path_seek(rule_base: &Path) -> Vec<Rule> {
    println!("[+] Rule base directory: {}", rule_base.display());
    // let mut last_result: Vec<PathBuf> = vec![rule_base.to_path_buf()];
    // let result = traverse_directories(rule_base, &mut last_result);
    // let mut rule: Vec<Rule> = vec![];
    // for path in result.iter() {
    //     if path.extension() == "json" {
    //         let data = read_file(path);
    //         rule.push(prase_json(data.as_str()));
    //     }
    // }
    // return rule;
    let extensions = vec![String::from("json")];
    let files: Vec<PathBuf> = iter_files(rule_base, extensions.clone())
        .map(|d| d.into_path())
        .collect();
    let mut rule: Vec<Rule> = vec![];
    for path in files.iter() {
        let data = read_file(path);
        rule.push(prase_json(data.as_str()));
    }
    return rule;
}

/// Recursively iterate through all files under `path` that match an ending listed in `extensions`
fn iter_files(path: &Path, extensions: Vec<String>) -> impl Iterator<Item = walkdir::DirEntry> {
    let is_hidden = |entry: &walkdir::DirEntry| {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    };

    WalkDir::new(path)
        .into_iter()
        .filter_entry(move |e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(move |entry| {
            if entry.file_type().is_dir() {
                return false;
            }

            let path = entry.path();

            match path.extension() {
                None => return false,
                Some(ext) => {
                    let s = ext.to_str().unwrap_or_default();
                    if !extensions.contains(&s.to_string()) {
                        return false;
                    }
                }
            }
            true
        })
}

pub fn code_path_seek(code_base: &Path, cpp: bool) -> Vec<PathBuf> {
    let path = if code_base.is_absolute() || code_base.to_string_lossy() == "-" {
        code_base.to_path_buf()
    } else {
        std::env::current_dir().unwrap().join(code_base)
    };
    // let mut last_result: Vec<PathBuf> = vec![code_base.to_path_buf()];
    let extensions = {
        if !cpp {
            vec!["c".to_string(), "h".into()]
        } else {
            vec![
                "cc".to_string(),
                "cpp".into(),
                "h".into(),
                "cxx".into(),
                "hpp".into(),
            ]
        }
    };
    // let mut result = traverse_directories(code_base, &mut last_result);
    // if cpp {
    //     let ext_check =
    //         |path: &PathBuf| path.extension().unwrap() == "c" || path.extension().unwrap() == "cpp";
    //     result.retain(ext_check);
    // } else {
    //     let ext_check = |path: &PathBuf| path.extension().unwrap() == "c";
    //     result.retain(ext_check);
    // }
    let files: Vec<PathBuf> = iter_files(&path, extensions.clone())
        .map(|d| d.into_path())
        .collect();
    println!("[+] parsing {} files", files.len());
    if files.is_empty() {
        eprintln!("{}", String::from("No files to parse. Exiting..."));
        std::process::exit(1)
    }
    return files;
}


// /// Helper function to parse an input string
// /// into a tree-sitter tree, using our own slightly modified
// /// C grammar. This function won't fail but the returned
// /// Tree might be invalid and contain errors.
// pub fn parse(source: &str, cpp: bool) -> Tree {
//     let mut parser = get_parser(cpp);
//     parser.parse(source, None).unwrap()
// }

// pub fn get_parser(cpp: bool) -> Parser {
//     let language = if !cpp {
//         unsafe { tree_sitter_c() }
//     } else {
//         unsafe { tree_sitter_cpp() }
//     };

//     let mut parser  = Parser::new();
//     if let Err(e) = parser.set_language(language) {
//         eprintln!("{}", e);
//         panic!();
//     }
//     parser
// }


