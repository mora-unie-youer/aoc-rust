{
  description = "aoc-rust - project which contains solutions to all AoC events in Rust";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

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
            # inputs.devshell.overlays.default
            inputs.rust-overlay.overlays.default
          ];
        };
      in {
        devShells.default = import ./devshell.nix { inherit pkgs; };
      }
    );
}
