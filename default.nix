let
  lock = builtins.fromJSON (builtins.readFile ./flake.lock);

  flake-compat = builtins.fetchTarball {
    url = "https://github.com/edolstra/flake-compat/archive/${lock.nodes.flake-compat.locked.rev}.tar.gz";
    sha256 = lock.nodes.flake-compat.locked.narHash;
  };

  flake = import flake-compat {
    src = ./.;
  };

  flakeOutputs = flake.defaultNix.outputs;

  output = flakeOutputs // {
    pkgs = flakeOutputs.legacyPackages.${builtins.currentSystem};
  };
in output
