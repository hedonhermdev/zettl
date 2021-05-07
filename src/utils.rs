use crate::config::Config;
use anyhow::Result;
use chrono::{DateTime, Local};
use heck::TitleCase;
use ignore::{DirEntry, Walk};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, fs, io::{BufWriter, Write}, path::{Path, PathBuf}, rc::Rc};

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
pub struct Link {
    source: Rc<String>,
    target: Rc<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    id: Rc<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

pub fn get_index_items(prefix: &Path, directory: &Path) -> (Vec<String>, Vec<PathBuf>) {
    let mut items: Vec<String> = vec![];
    let mut dirs: Vec<PathBuf> = vec![];

    let mut paths: Vec<PathBuf> = directory
        .read_dir()
        .unwrap()
        .map(|entry| {
            let entry = entry.expect("Failed to read DirEntry");

            let path = entry.path();

            path
    }).collect();

    paths.sort_by_key(|p| { fs::metadata(p).unwrap().modified().unwrap()});
    paths.reverse();

    paths
        .iter()
        .for_each(|path| {
        let meta = fs::metadata(path).unwrap();
        let relpath = path.strip_prefix(prefix).unwrap();
        if relpath.starts_with(".") {
            return;
        }
        if meta.is_dir() {
            let mut item = relpath.to_string_lossy().to_string();
            item.push_str("/_index");
            items.push(item);
            dirs.push(path.to_path_buf());
        }

        if meta.is_file() && path.extension() == Some(OsStr::new("md")) && path.file_stem() != Some(OsStr::new("_index")) {
            items.push(
                relpath
                    .to_string_lossy()
                    .to_string()
                    .strip_suffix(".md")
                    .unwrap()
                    .to_owned(),
            );
        }
    });

    (items, dirs)
}

pub fn write_index_file(cfg: &Config, base: &Path, cur: &Path) -> Result<()> {
    let (items, dirs) = get_index_items(base, cur);

    println!("{:#?}", &items);

    let index_file = cur.join(Path::new("_index.md"));
    let dirname = cur
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();

    let title = format!("{} Index", dirname.to_title_case());
    let front_matter = FrontMatter {
        title: &title,
        author: &cfg.author,
        created: Local::now(),
    };

    let mut contents = serde_yaml::to_string(&front_matter)?;
    contents.push_str("---\n");

    let mut buf = BufWriter::new(fs::File::create(index_file)?);

    // Write frontMatter

    contents.push_str(&format!("\n# {}\n\n", title));

    for entry in items {
        if entry.starts_with('.') {
            continue;
        }
        contents.push_str(&format!("- [[{}]]\n", entry));
    }

    for dir in dirs {
        write_index_file(cfg, base, &dir)?;
    }

    buf.write_all(contents.as_bytes())?;

    Ok(())
}

pub fn update_index(cfg: &Config, directory: &Path) -> Result<()> {
    write_index_file(cfg, directory, directory)
}

pub fn open_file_in_editor(
    cfg: &Config,
    basedir: &Path,
    file: &Path,
) -> Result<subprocess::ExitStatus> {
    cfg.editor_cmd.split(' ');
    let exec = subprocess::Exec::cmd(cfg.editor_cmd.clone())
        .args(&cfg.editor_args)
        .arg(file.as_os_str())
        .cwd(basedir);
    let exit_status = exec.join()?;

    Ok(exit_status)
}

pub fn write_skeleton(file: &Path, front_matter: &FrontMatter) -> Result<()> {
    let mut fm = serde_yaml::to_string(&front_matter)?;
    fm.push_str("---\n");

    let mut buf = BufWriter::new(fs::File::create(file)?);

    // Write frontMatter
    buf.write_all(fm.as_bytes())?;

    let heading = format!("\n# {}\n", front_matter.title);

    buf.write_all(heading.as_bytes())?;

    Ok(())
}

pub fn update_graph(directory: &Path) -> Result<()> {
    let re = Regex::new(r"\[\[([^\]\[]+)\]\]").unwrap();

    let files: Vec<DirEntry> = Walk::new(directory)
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().extension() == Some(OsStr::new("md")))
        .collect();

    let targets: Vec<Rc<String>> = files
        .iter()
        .map(|f| {
            Rc::new(
                f.path()
                .strip_prefix(directory)
                .unwrap()
                .to_str()
                .unwrap()
                .strip_suffix(".md")
                .unwrap()
                .to_owned()
            )
        })
        .collect();

    let nodes = targets.iter().map(|t| Node { id: t.clone()}).collect();

    let mut graph = Graph {
        nodes,
        links: vec![],
    };

    files.iter().for_each(|f| {
        let text = fs::read_to_string(f.path()).expect("Could not read file");

        let source = Rc::new(
            f
            .path()
            .strip_prefix(directory)
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".md")
            .unwrap()
            .to_owned()
        );

        for m in re.find_iter(&text) {
            let cap = re.captures(m.as_str()).unwrap().get(1).unwrap();
            if let Some(target) = targets.iter().find(|n| n.as_str() == cap.as_str()) {
                let link = Link {
                    source: source.clone(),
                    target: target.clone(),
                };
                graph.links.push(link);
            }
        }
    });

    let ser = serde_json::to_vec(&graph)?;

    fs::File::create(directory.join(".graph.json"))?.write_all(&ser)?;

    Ok(())
}
