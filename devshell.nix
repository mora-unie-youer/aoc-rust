{ pkgs, lib ? pkgs.lib }:
configFile:

with lib;
let
  moduleConfiguration = {
    options = {
      name = mkOption {
        type = types.str;
        description = "Name of devshell";
      };

      packages = mkOption {
        type = types.listOf types.package;
        description = "Packages which will be included in devshell";
      };
    };

    config = {
      name = mkDefault "aoc-rust";
      packages = with pkgs; [
        # Toolchain needed to build binaries
        binutils
        gcc
        # Rust toolchain
        bacon
        hyperfine
        (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-analyzer" "rust-src" ];
        }))
        # Binaries and libraries needed for Rust crates
        openssl
        pkg-config
      ];
    };
  };

  module = evalModules {
    modules = [
      configFile
      moduleConfiguration
    ];
  };
in
pkgs.devShell.mkShell {
  inherit (module.config) name packages;
  preInitShellHook = ''
    cd ${module.config.name}
  '';
}
