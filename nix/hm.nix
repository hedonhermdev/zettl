{ config, pkgs, lib ? pkgs.lib, ... }:

let 
  cfg = config.programs.zettl;
  zettlBuilder = pkgs.zettlBuilder { config = cfg; };
in 
with lib; {
  options.programs.zettl = {
    enable = mkEnableOption "A blazing fast note-taking sytem";

    settings = mkOption {
      type = types.attrsOf types.anything;
      default = { };
      example = literalExpression ''
        {
          zettlDir = "/home/tirth/kasten";
          author = "Tirth Jain";
          editorCmd = "${pkgs.neovim}/bin/nvim";
          editorArgs = [ ];
          indexes = true;
          graph = true;
        }
      '';
      description = "Options for your zettl notebook. ";
    };
  };
}
