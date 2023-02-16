{ pkgs, ... }:

configFile:
let
  # Default devshell configuration
  defaultConfig = {
    # Name of devshell
    name = "aoc-rust";
    # Packages used in devshell
    packages = with pkgs; [
      # Toolchain needed to build binaries
      binutils
      gcc
      # Rust toolchain
      (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
        extensions = [ "rust-analyzer" "rust-src" ];
      }))
      # Binaries and libraries needed for Rust crates
      openssl
      pkg-config
    ];
  };

  # Function to merge default and given configurations
  recursiveMerge = attrList:
    with pkgs.lib;
    let f = attrPath:
      zipAttrsWith (n: values:
        if tail values == []
          then head values
        else if all isList values
          then unique (concatLists values)
        else if all isAttrs values
          then f (attrPath ++ [n]) values
        else last values
      );
    in f [] attrList;

  loadedConfiguration = import configFile { inherit pkgs; };

  # Devshell configuration
  configuration = recursiveMerge [
    defaultConfig
    loadedConfiguration
    {
      shellHooks.preInit = ''
        cd ${loadedConfiguration.name}
      '';
    }
  ];
in
pkgs.devShell.mkShell configuration
