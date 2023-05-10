{
  description = "aoc-rust - project which contains solutions to all AoC events in Rust";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    devshell = {
      url = "github:mora-unie-youer/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = inputs:
    inputs.flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.devshell.overlays.default
            inputs.rust-overlay.overlays.default
          ];
        };

        devShell = import ./devshell.nix { inherit pkgs; };
      in rec {
        devShells = {
          "2015" = devShell ./aoc-2015/devshell.nix;
          "2016" = devShell ./aoc-2016/devshell.nix;
          "2017" = devShell ./aoc-2017/devshell.nix;
          "2018" = devShell ./aoc-2018/devshell.nix;
          "2019" = devShell ./aoc-2019/devshell.nix;
          default = devShells."2019";
        };
      }
    );
}
