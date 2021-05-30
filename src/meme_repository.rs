#![allow(dead_code)]

use colored::*;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

type FormatMap = HashMap<String, MemeFormat>;
type InsertsMap = HashMap<String, ((u32, u32), (u32, u32))>;

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
    /// inserts and their coordinates
    pub inserts: InsertsMap,
}

const PRINT_REPO_DEBUG: bool = true;

impl FormatRepo {
    pub fn new(root_path: PathBuf, name: String) -> Result<FormatRepo, io::Error> {
        let mut formats_map = FormatMap::new();

        // check assertions
        assert!(root_path.exists(), "Format repo root path does not exist.");
        assert!(root_path.is_dir(), "Format repo root path not a directory.");
        if PRINT_REPO_DEBUG {
            println!(
                "  {:41.41}  {:52.52}  {:20.20}",
                "IDENTIFIER", "IMAGE PATH", "METADATA"
            );
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
            // don't process metadata files
            if e.path().extension().unwrap_or(OsStr::new("")).eq("meme") {
                continue;
            }

            // get the path into a vector
            let mut path_vec: Vec<&str> = e.path().iter().map(|s| s.to_str().unwrap()).collect();
            path_vec[0] = &name; // make the first part say "Meme"
            let last_index = path_vec.len() - 1; // remove the file extension
            let last_elem = path_vec.last_mut().unwrap();
            path_vec[last_index] = &mut &last_elem[..last_elem.find('.').unwrap()];

            // check if we have metadata available
            let metadata = get_metadata(&e.path());
            let inserts = metadata.clone().unwrap_or(InsertsMap::new());

            // form the string
            let memeid = path_vec.join(".");
            if PRINT_REPO_DEBUG {
                println!(
                    ") {:41.41}  {:52.52}  {:20.20}",
                    memeid,
                    e.path().display(),
                    match &metadata {
                        Err(e) => e.to_string(),
                        Ok(ins) => format!("{} inserts", ins.len()),
                    }
                );
            }

            // add to the hashmap
            formats_map.insert(
                memeid.clone(),
                MemeFormat {
                    memeid,
                    image_path: e.path().to_path_buf(),
                    inserts,
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

fn get_metadata(meme_image_path: &Path) -> Result<InsertsMap, &str> {
    let metadata_path = meme_image_path.with_extension("meme");
    if !metadata_path.exists() {
        return Err("None");
    };
    let json: serde_json::Value = serde_json::from_reader(std::io::BufReader::new(
        std::fs::File::open(&metadata_path).unwrap(),
    ))
    .expect(&format!("Failed to parse meme metadata JSON {:?}.", metadata_path).to_string());
    let inserts_json = json
        .get("inserts")
        .ok_or_else(|| "No inserts")?
        .as_object()
        .ok_or_else(|| "Inserts not object")?;
    let mut inserts = InsertsMap::new();
    for (ins_name, ins_val) in inserts_json {
        let coords_arr = ins_val
            .get("coords")
            .ok_or("No coords")?
            .as_array()
            .ok_or("Coords not array")?;
        if coords_arr.len() != 2 {
            return Err("Not coords pair");
        }
        let cs1: Vec<u32> = coords_arr[0]
            .as_array()
            .ok_or("Coords[0] not array")?
            .into_iter()
            .map(|c| c.as_u64().unwrap_or(0) as u32)
            .collect();
        let cs2: Vec<u32> = coords_arr[1]
            .as_array()
            .ok_or("Coords[1] not array")?
            .into_iter()
            .map(|c| c.as_u64().unwrap_or(0) as u32)
            .collect();
        inserts.insert(ins_name.clone(), ((cs1[0], cs1[1]), (cs2[0], cs2[1])));
    }
    Ok(inserts)
}
