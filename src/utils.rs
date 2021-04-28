use crate::config::Config;
use anyhow::Result;
use chrono::{DateTime, Local};
use heck::TitleCase;
use ignore::Walk;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{BufWriter, Write},
    path::Path,
};

mod my_date_format {
    use chrono::{DateTime, Local, TimeZone};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Local
            .datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrontMatter<'a> {
    pub title: &'a str,
    pub author: &'a str,
    #[serde(with = "my_date_format")]
    pub created: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    source: String,
    target: String
}

pub type Node = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>
}

pub fn update_index(cfg: &Config, directory: &Path) -> Result<()> {
    let index_entries: Vec<String> = directory.read_dir()
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            let mut index_entry = entry
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            if entry.file_type().unwrap().is_dir() {
                update_index(cfg, entry.path().as_path()).unwrap();
                index_entry.push_str("/_index")
            }

            index_entry
        }).collect();


    let index_file = directory.join(Path::new("_index.md"));
    // Fix this
    let dirname = directory
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();

    let title = dirname.to_title_case();

    let front_matter = FrontMatter {
        title: &title,
        author: &cfg.author,
        created: Local::now(),
    };

    let mut fm = serde_yaml::to_string(&front_matter)?;
    fm.push_str("---\n");

    let mut buf = BufWriter::new(fs::File::create(index_file)?);

    // Write frontMatter
    buf.write(fm.as_bytes())?;

    buf.write(format!("\n# {}\n\n", title).as_bytes())?;

    if !index_entries.is_empty() {
        for entry in index_entries {
            if entry.starts_with(".") || entry == "_index" {
                continue;
            }
            buf.write(format!("- [[{}]]\n", entry).as_bytes())?;
        }
    }

    Ok(())
}

pub fn open_file_in_editor(
    cfg: &Config,
    basedir: &Path,
    file: &Path,
) -> Result<subprocess::ExitStatus> {
    cfg.editor_cmd.split(" ");
    println!("{:#?}", &cfg.editor_args);
    let exec = subprocess::Exec::cmd(cfg.editor_cmd.clone())
        .args(&cfg.editor_args)
        .arg(file.as_os_str())
        .cwd(basedir);
    println!("{:#?}", exec);
    let exit_status = exec.join()?;

    Ok(exit_status)
}

pub fn write_skeleton(file: &Path, front_matter: &FrontMatter) -> Result<()> {
    let mut fm = serde_yaml::to_string(&front_matter)?;
    fm.push_str("---\n");

    let mut buf = BufWriter::new(fs::File::create(file)?);

    // Write frontMatter
    buf.write(fm.as_bytes())?;

    let heading = format!("\n# {}\n", front_matter.title);

    buf.write(heading.as_bytes())?;

    Ok(())
}

pub fn update_graph(directory: &Path) -> Result<()> {

    let re = Regex::new(r"\[\[([^\]\[]+)\]\]").unwrap();

    let mut graph =  Graph {
        nodes: vec![],
        edges: vec![]
    };

    Walk::new(directory)
        .for_each(|entry| {
            let entry = entry.unwrap();
            if entry.file_name().to_str().unwrap().ends_with(".md") {
                let node = entry.path().strip_prefix(directory).unwrap().to_str().unwrap().to_owned();

                graph.nodes.push(node.clone());

                let text = fs::read_to_string(entry.path()).expect("Unable to read file. Check permissions");

                for m in re.find_iter(&text) {
                    let cap = re.captures(m.as_str()).unwrap().get(1).unwrap();
                    let edge = Edge{
                        source: node.clone(),
                        target: cap.as_str().to_owned()
                    };

                    graph.edges.push(edge);
                }
            }
        });

    let ser = serde_json::to_vec(&graph)?;

    fs::File::create(directory.join(".graph.json"))?.write(&ser)?;

    Ok(())
}
