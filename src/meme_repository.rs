#![allow(dead_code)]

use colored::*;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use walkdir::WalkDir;

type FormatMap = HashMap<String, MemeFormat>;

pub struct FormatRepo {
    /// top level name of the repo
    pub name: String,
    /// map containing all the memes by their fully qualified memeIDs
    pub formats: FormatMap,
}

pub struct MemeFormat {
    /// fully qualified memeID
    pub memeid: String,
    /// path to the image containing this format
    pub image_path: PathBuf,
}

const PRINT_REPO_DEBUG: bool = true;

impl FormatRepo {
    pub fn new(root_path: PathBuf, name: String) -> Result<FormatRepo, io::Error> {
        let mut formats_map = FormatMap::new();

        // check assertions
        assert!(root_path.exists(), "Format repo root path does not exist.");
        assert!(root_path.is_dir(), "Format repo root path not a directory.");
        if PRINT_REPO_DEBUG {
            println!("  {:42.42} {}", "IDENTIFIER", "IMAGE PATH");
        }

        // do recursive traversal
        for entry in WalkDir::new(root_path.clone())
            .min_depth(1)
            .sort_by(|a, b| a.file_name().cmp(b.file_name()))
            .into_iter()
        {
            let e = entry.unwrap();
            // only process files, not directories
            if !e.file_type().is_file() {
                continue;
            }
            // get the path into a vector
            let mut path_vec: Vec<&str> = e.path().iter().map(|s| s.to_str().unwrap()).collect();
            path_vec[0] = &name; // make the first part say "Meme"
            let last_index = path_vec.len() - 1; // remove the file extension
            let last_elem = path_vec.last_mut().unwrap();
            path_vec[last_index] = &mut &last_elem[..last_elem.find('.').unwrap()];
            // form the string
            let memeid = path_vec.join(".");
            if PRINT_REPO_DEBUG {
                println!(") {:42.42} {}", memeid, e.path().display());
            }
            // add to the hashmap
            formats_map.insert(
                memeid.clone(),
                MemeFormat {
                    memeid,
                    image_path: e.path().to_path_buf(),
                },
            );
        }

        // gloat
        println!(
            "Repository loaded with {} memes.",
            formats_map.len().to_string().bold()
        );

        // create and return the struct
        Ok(FormatRepo {
            name,
            formats: formats_map,
        })
    }
}
