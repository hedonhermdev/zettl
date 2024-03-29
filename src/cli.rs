use anyhow::{Context, Error, Result};
use std::{path::Path, path::PathBuf};
use structopt::StructOpt;
use tokio::fs;

use crate::commands::{fleet, graph, index, init, note, list};

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "init", about = "Initialize zettl")]
    Init,

    #[structopt(name = "fleet", about = "Create a new fleeting note")]
    Fleet {
        #[structopt(
            short = "o",
            long = "open",
            about = r#"Name of the fleeting note to open.
                       If value given, will open the fleeting note if present.
                       Otherwise it will open/create a fleeting note for the current day.
                    "#
        )]
        name: Option<PathBuf>,
    },

    #[structopt(name = "note", about = "Create a new note")]
    Note {
        #[structopt(
            name = "NAME",
            about = "Name to give your note. This can contain a path like apple/pen."
        )]
        name: PathBuf,
    },

    #[structopt(name = "list", about = "List all notes.")]
    List {
        #[structopt(
            long = "fleet",
            short = "f",
            about = "Show fleeting notes if set."
        )]
        fleet: bool,
    },

    #[structopt(name = "index", about = "Create indexes.")]
    Index,

    #[structopt(name = "graph", about = "Create graphs.")]
    Graph,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "zettl", about = "A blazing fast note-taking system")]
pub struct CLI {
    #[structopt(
        name = "basedir",
        long,
        env = "ZETTL_DIRECTORY",
        default_value = "~/zettel"
    )]
    basedir: PathBuf,
    #[structopt(
        name = "config-file",
        long,
        env = "ZETTL_CFG",
    )]
    cfg_file: Option<PathBuf>,
    #[structopt(subcommand)]
    command: Command,
}

impl CLI {
    pub async fn run() -> Result<()> {
        let args = Self::from_args();

        // Sanitize base dir
        let mut basedir = args.basedir;

        if basedir.as_path() == Path::new("~") {
            basedir = dirs::home_dir().ok_or_else(|| Error::msg("Invalid path"))?;
        }

        if basedir.starts_with("~/") {
            let home_dir = dirs::home_dir().ok_or_else(|| Error::msg("Invalid path"))?;

            basedir = basedir.strip_prefix("~/")?.to_path_buf();

            basedir = home_dir.join(basedir);
        }

        fs::create_dir_all(&basedir).await.context("Could not create base directory")?;

        let basedir = basedir.canonicalize().context("Invalid base directory")?;

        let default_cfg_file = basedir.clone().join(".zettl/config.yml");

        let cfg_file = args.cfg_file.unwrap_or(default_cfg_file);


        // Match and execute command
        use Command::*;
        match args.command {
            Init => init(&basedir)
                .await
                .context("Failed to initialize in the given base directory."),

            Fleet { name } => fleet(&basedir, &cfg_file, name.as_deref())
                .await
                .context("Failed to open fleet."),

            Note { name } => note(&basedir, &cfg_file, name.as_path())
                .await
                .context("Failed to open note with the given name"),

            Index => index(&basedir, &cfg_file)
                .await
                .context("Failed to index notes."),

            Graph => graph(&basedir)
                .await
                .context("Failed to create graph of notes"),

            List { fleet } => list(&basedir, fleet)
                .await
                .context("Failed to list notes"),
        }
    }
}

