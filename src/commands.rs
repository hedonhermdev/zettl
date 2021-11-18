use heck::TitleCase;
use std::path::{Path, PathBuf};

use tokio::fs;

use anyhow::{Context, Error, Result};

use crate::utils::{update_graph, update_index};
use crate::{
    config::Config,
    utils::{open_file_in_editor, write_skeleton, FrontMatter},
};

/// Initialize the Zettl directory with the config etc.
pub async fn init(basedir: PathBuf) -> Result<()> {
    // Create config dir
    let cfg_dir = basedir.join(Path::new(".zettl"));
    fs::create_dir(cfg_dir.as_path())
        .await
        .context("Failed to create config directory")?;

    // Create fleets dir
    let fleets_dir = basedir.join(Path::new("fleets"));
    fs::create_dir(fleets_dir)
        .await
        .context("Failed to create fleets directory")?;

    // Create notes dir
    let notes_dir = basedir.join(Path::new("notes"));
    fs::create_dir(notes_dir)
        .await
        .context("Failed to create notes directory")?;

    // Store default config
    let cfg = Config::default();
    let ser = cfg.serialize().context("Failed to serialize context")?;
    let cfg_file = basedir.join(Path::new(".zettl/config.yml"));
    fs::write(cfg_file, ser)
        .await
        .context("Failed to write default config file")?;

    // Create base index
    if cfg.indexes {
        update_index(&cfg, &basedir).await.context("Failed to create _index.md")?;
    }

    // Create graph
    if cfg.graph {
        update_graph(&basedir).await.context("Failed to create .graph.json")?;
    }

    Ok(())
}

pub async fn fleet(basedir: PathBuf, name: Option<PathBuf>) -> Result<()> {
    let cfg_file = basedir.join(CONFIG_DIR).join(CONFIG_FILE);

    let cfg = Config::from_file(&cfg_file).context("Cannot read config file")?;

    let fleet_file = match name {
        Some(name) => {
            let filepath = format!("fleets/{}.md", name.to_str().unwrap());
            let fleet_file = basedir.join(filepath);
            if !fleet_file.exists() {
                anyhow::bail!("Fleeting note doesn't exist");
            }
            fleet_file
        }
        None => {
            let now = chrono::Local::now();

            let today = now.date().format("%Y-%m-%d");
            let today_title = now.date().format("%A, %d %B %Y");

            let filepath = format!("fleets/{}.md", &today);
            let title = format!("{}", &today_title);
            let fleet_file = basedir.join(filepath);

            if !fleet_file.exists() {
                let front_matter = FrontMatter {
                    title: &title,
                    author: &cfg.author,
                    created: now,
                };

                write_skeleton(&fleet_file, &front_matter).await?;
            }
            fleet_file
        }
    };

    open_file_in_editor(&cfg, basedir.as_path(), &fleet_file)
        .context("Could not open file in editor")?;

    if cfg.indexes {
        update_index(&cfg, &basedir).await.context("Failed to create _index.md")?;
    }

    if cfg.graph {
        update_graph(&basedir).await?;
    }

    Ok(())
}


pub async fn note(basedir: PathBuf, name: PathBuf) -> Result<()> {
    let cfg_file = basedir.join(".zettl/config.yml");

    let cfg = Config::from_file(&cfg_file).context("Cannot read config file")?;
    let now = chrono::Local::now();

    let note_file = basedir
        .join("notes")
        .join(&format!("{}.md", name.to_str().unwrap()));
    if let Some(note_dir) = note_file.parent() {
        fs::create_dir_all(note_dir).await?;
    };

    if !(note_file.exists()) {
        let title = &note_file
            .file_stem()
            .ok_or_else(|| Error::msg("Invalid note name"))?
            .to_string_lossy()
            .to_title_case();

        let front_matter = FrontMatter {
            title,
            author: &cfg.author,
            created: now,
        };

        write_skeleton(&note_file, &front_matter).await?;
    }

    open_file_in_editor(&cfg, basedir.as_path(), &note_file)
        .context("Could not open file in editor")?;

    if cfg.indexes {
        update_index(&cfg, &basedir).await.context("Failed to create _index.md")?;
    }

    if cfg.graph {
        update_graph(&basedir).await.context("Failed to create .graph.json")?;
    }

    Ok(())
}

pub async fn index(basedir: PathBuf) -> Result<()> {
    let cfg_file = basedir.join(".zettl/config.yml");

    let cfg = Config::from_file(&cfg_file).context("Cannot read config file")?;

    update_index(&cfg, &basedir).await?;

    Ok(())
}

pub async fn graph(basedir: PathBuf) -> Result<()> {
    update_graph(&basedir).await.context("Failed to create .graph.json")?;

    Ok(())
}
