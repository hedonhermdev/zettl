{ pkgs, lib ? pkgs.lib, ... }:

{ config }:

let
  zettlOptions = lib.evalModules {
    modules = [
      (import ./zettl.nix)
      config
    ];
  };

  zettl = zettlOptions.config.zettl;

  zettlDir = zettl.zettlDir;

  editorArgs = lib.concatStringsSep " " zettl.editorArgs;

  zettlConfig = ''
    ---
    name: ${zettl.name}
    author: ${zettl.author}
    editor_cmd: ${zettl.editorCmd}
    editor_args: [ ${editorArgs} ]
    indexes: ${if zettl.indexes then "true" else "false" }
    graph: ${if zettl.graph then "true" else "false" }
  '';

  zettlCfg = pkgs.writeText "config.yml" zettlConfig;

in

pkgs.rustPlatform.buildRustPackage {
    pname = "zettl-unwrapped";

    version = "0.0.1";

    src = ../.;

    buildInputs = [ pkgs.makeWrapper ];
    cargoLock.lockFile = ../Cargo.lock;

    postInstall = ''
      wrapProgram "$out/bin/zettl" \
      --set ZETTL_DIRECTORY ${zettlDir} \
      --set ZETTL_CFG ${zettlCfg}
    '';
}
