{ pkgs, ... }:

configFile:
let
  # Default devshell configuration
  defaultConfig = {
    # Name of devshell
    name = "aoc-rust";
    # 3-bit/8-bit/24-bit coloring of devshell
    colors = {
      "3bit" = "1";
      "8bit" = "202";
      "24bit" = "#17BEBB";
    };
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

  # Devshell configuration
  config = recursiveMerge [
    defaultConfig
    (import configFile { inherit pkgs; })
  ];
in
pkgs.mkShell {
  buildInputs = config.packages;

  shellHook = ''
    # Initialize LD path for library loading
    export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath config.packages}
    # Moving to AoC <year> folder
    cd ${config.name}
    PROJECT_ROOT=$PWD

    rel_root() {
      local path
      path=$(${pkgs.coreutils}/bin/realpath --relative-to $PROJECT_ROOT $PWD)
      if [[ $path != . ]]; then echo " $path "; fi
    }

    if [[ $COLORTERM = "truecolor" ]] || [[ $COLORTERM = "24bit" ]]; then
      # We have 24-bit colors
      HEX_COLOR=${config.colors."24bit"}
      COLOR=$(printf "\e[38;2;%d;%d;%d2m" 0x''${HEX_COLOR:1:2} 0x''${HEX_COLOR:3:2} 0x''${HEX_COLOR:5:2})
    else
      _colors=$(tput colors)
      if [[ "$_colors" = "256" ]]; then
        # We have 8-bit colors
        COLOR=$(printf "\e[38;5;%dm" ${config.colors."8bit"})
      elif [[ "$_colors" = "8" ]] || [[ "$_colors" = "16" ]]; then
        # We have 3/4-bit colors
        COLOR=$(printf "\e[3%dm" ${config.colors."3bit"})
      fi
    fi
    PS1="\[$COLOR\][${config.name}]\$(rel_root)\$\[\033[0m\] "
  '';
}
