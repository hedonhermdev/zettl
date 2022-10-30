{
  description = "Zettl is a note-taking system";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }@inputs:
    let
      supportedSystems = [ "aarch64-darwin" "aarch64-linux" "x86_64-linux" ];
    in
    flake-utils.lib.eachSystem supportedSystems
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            config = { allowUnfree = true; };

            overlays = [
              (import rust-overlay)
            ];
          };

          zettlBuilder = (import ./nix pkgs);

          zettl = (zettlBuilder {
            config.zettl = {
              zettlDir = "/home/tirth/kasten";
              author = "Tirth Jain";
              editorCmd = "${pkgs.neovim}/bin/nvim";
              editorArgs = [ ];
              indexes = true;
              graph = true;
            };
          });

        in
        rec {
          apps = {
            zettl = {
              type = "app";
              program = [ packages.zettl ];
            };
          };

          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [
              rust-bin.beta.latest.default
              zettl
            ];
          };

          packages = rec {
            inherit zettl zettlBuilder;
            default = zettl;
          };

          nixosModules.hm = rec {
            imports = [
              ./nix/hm.nix
              { nixpkgs.overlays = [ self.overlays.default ]; }
            ];
          };
        }) // {
      overlays.default = final: prev: {
        zettl = self.packages.${final.system}.zettl;
        zettlBuilder = self.packages.${final.system}.zettlBuilder;
      };
    };

}
