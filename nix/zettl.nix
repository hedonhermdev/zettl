{ config, pkgs, lib ? pkgs.lib, ... }:

with lib;
with builtins;

let
  cfg = config.zettl;
in
{
  options = {
    zettl = {
      zettlDir = mkOption {
        description = "path to the zettl notebook";
        type = types.string;
        default = "";
      };

      name = mkOption {
        description = "name of the zettl notebook";
        type = types.string;
        default = "My Zettelkasten";
      };

      author = mkOption {
        description = "name of the zettl author";
        type = types.string;
        default = "My Zettelkasten";
      };

      editorCmd = mkOption {
        description = "the command to use to spawn the editor";
        type = (types.either types.string types.path);
        default = "${pkgs.neovim}/bin/nvim";
      };

      editorArgs = mkOption {
        description = "extra args to be passed to the editor";
        type = (types.listOf types.string);
        default = [ ];
      };

      graph = mkOption {
        description = "wether to make the `graph.json` file or not";
        type = types.bool;
        default = true;
      };

      indexes = mkOption {
        description = "wether to make the `index.md` file or not";
        type = types.bool;
        default = true;
      };
    };
  };

  config = {
    zettl = {};
  };
}
