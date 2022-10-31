# Zettl
⚡️A blazing fast way of maintaining powerful notes with connections between them.

[![asciicast](https://asciinema.org/a/vSIiOqDOEsIuGuOgTOMlBPTN3.svg)](https://asciinema.org/a/vSIiOqDOEsIuGuOgTOMlBPTN3)

(i know its broken im too lazy to re-record)

## Installing Zettl

To install Zettl, you will need the Rust toolchain installed. You can install Rust using [rustup](https://rustup.rs). Once you have Rust installed, you can either install zettl from [crates.io](https://crates.io).
```bash
$ cargo install zettl
```

### Using Home Manager
Zettl now ships as a Nix flake with a `home-manger` module. Since zettl is not part of Nixpkgs (yet!), you need to add the zettl overlay to your nixpkgs. 

If you use a flake to manager your NixOS configuration, you can add the following to your `flake.nix`
```nix
  inputs = {
   zettl.url = "github:hedonhermdev/zettl";
  };

  ...
  pkgs = import nixpkgs {
    inherit system;
    config = {
      allowUnfree = true;
    };

    overlays = [
      zetl.overlays.default
    ];
  };
```

And to add it to your home programs, you can do something like: 

```nix
programs.zettl = {
  enable = true;
  settings = {
    zettl = {
      zettlDir = "${config.home.homeDirectory}/kasten";
      editorCmd = "${pkgs.nvimPacked}/bin/nvim";
      author = "Tirth Jain";
      name = "zettelkasten";
    };
  };
};
```


## Initializing Zettl
You will need to create a directory to store your notes. You can tell zettl to use this directory  by setting the `$ZETTL_DIRECTORY` variable. Note that zettl will use this directory for all operations so you will probably have to set this variable in your `.bashrc` (or your `.zshrc`).

```bash
$ mkdir ~/kasten
$ echo "export ZETTL_DIRECTORY=~/kasten" >> .bashrc
```

Initializing zettl creates a `.zettl` directory in your base directory.
```bash
$ zettl init
```

## Configuring Zettl
You can change config options by setting values in the config file.
```
$ vim $ZETTL_DIRECTORY/.zettl/config.yml
```

Currently, zettl supports the following options:

```yaml
---
name: My Zettelkasten
author: Me
editor_cmd: vim
editor_args: []
indexes: true
graph: true
```

An example, customised config will look like this:

```yaml
---
name: My Zettelkasten
author: Tirth Jain
editor_cmd: nvim
editor_args:
  - "+Goyo"
  - "+Limelight"
indexes: true
graph: true
```

## Using Zettl

Once Zettl is initialized, you can use it to write notes from anywhere.

```bash
# Create a new fleeting note. These are like daily notes.
$ zettl fleet
# Create a new note. This will be created in notes/some-idea.md. 
$ zettl note some-idea
# Create a new note in a category. This will be created in notes/project1/some-idea.md.
$ zettl note project1/some-idea
# Listing all notes.
$ zettl list
# Listing all fleeting notes.
$ zettl list -f
```
These commands will open a markdown file in the editor you specified. 


### Graphs and Indexes

Zettl creates `_index.md` files in each directory to index your notes. You can turn this off by setting the following in the config directory.

```yaml
indexes: false
```

The `fleets/_index.md` file will look kind of like this:

```md
---
title: Fleets Index
author: Tirth Jain
created: "2021-04-29 11:16:25"
---

# Fleets Index

- [[fleets/2021-04-28]]
- [[fleets/2021-04-29]]
```

Similarly, Zettl creates a `.graph.json` file to track connections between your notes. You can visualize this graph with a visualizer of your choice. I prefer [3d-force-graph](https://github.com/vasturiano/3d-force-graph). Note that connections are made using the [[mediawiki]] link format. 

To turn off this graph generation: 
```yaml
graph: false
```

To manually create the graph and the indexes, you can run:

```bash
$ zettl graph
$ zettl index
```

## How Your Notes are Saved
Zettl saves your notes as markdown files. After a few days of using zettl, your zettl directory will look something like this:
```
/Users/hedonhermdev/kasten
├── _index.md
├── fleets
│   ├── 2021-04-28.md
│   ├── 2021-04-29.md
│   └── _index.md
└── notes
    ├── SDNs
    │   ├── _index.md
    │   └── p4-notes.md
    ├── _index.md
    ├── devops
    │   ├── _index.md
    │   ├── git-server-setup.md
    │   └── homegrown-autodeployment.md
    ├── git
    │   ├── _index.md
    │   ├── cli.md
    │   ├── error-handling.md
    │   ├── git-in-rust.md
    │   ├── implementation.md
    │   └── internal-objects.md
    ├── how-to-zettel.md
    ├── vim
    │   ├── _index.md
    │   ├── autocmds.md
    │   └── vimrc-for-servers.md
    ├── workflow.md
    ├── zettl
    │   ├── _index.md
    │   ├── philosophy.md
    │   └── roadmap.md
```

## Integrations

While zettl doesnt have a programmatic way to support integrations (yet!) but here are some integrations I can think of:

- Version control with git
```bash
$ cd $ZETTL_DIRECTORY
$ git init
$ git add .
$ git commit -m "Saving notes"
```

- Publish your notes as a static site with [Hugo](https://gohugo.io). This [theme](https://github.com/crisrojas/Zettels) handles mediawiki links as well. 
```bash
$ ln -s $ZETTL_DIRECTORY path/to/hugo/content
$ hugo serve
```

- Fuzzy finding notes with [fzf](https://github.com/junegunn/fzf)
```bash
# for notes
$ zettl note $(zettl list | fzf)
```
