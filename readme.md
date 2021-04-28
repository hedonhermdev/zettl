# Zettl
⚡️A blazing fast way of maintaining powerful notes with connections between them.

## Installing Zettl

To install Zettl, you will need the Rust toolchain installed. You can install Rust using [rustup](https://rustup.rs). Once you have Rust installed, clone the repository and install the binary using cargo.

```

Or you can build with the source

```bash

$ git clone https://github.com/hedonhermdev/zettl
$ cargo install --path zettl
```

## Initializing Zettl
You will need to create a directory to store your notes. You can tell zettl to use this directory  by setting the `$ZETTL_DIRECTORY` variable. Note that zettl will use this directory for all operations so you will probably have to set this variable in your `.bashrc` (or your `.zshrc`).

```bash
$ mkdir ~/kasten
$ echo "ZETTL_DIRECTORY=~/kasten" >> .bashrc
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
# Create a new fleeting note
$ zettl fleet
# Create a new note
$ zettl note some-idea
# Create a new note in a category
$ zettl note project1/some-idea
```

These commands will open a markdown file in the editor you specified. 


### Graphs and Indexes

Zettl creates `_index.md` files in each directory to index your notes. You can turn this off by setting the following in the config directory.

```yaml
indexes: false
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
