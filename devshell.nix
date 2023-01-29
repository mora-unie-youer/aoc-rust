{ pkgs, ... }:

let
  # Until I have nice devshell flake
  name = "aoc-rust";
  # Contains values for 3-bit/8-bit/24-bit coloring
  colors = {
    "3bit" = "1";
    "8bit" = "202";
    "24bit" = "#17BEBB";
  };
  packages = with pkgs; [
    # Toolchain needed to build binaries
    binutils
    gcc
    # Rust toolchain
    (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
      extensions = [ "rust-analyzer" "rust-src" ];
    }))
  ];
in
pkgs.mkShell {
  buildInputs = packages;

  shellHook = ''
    # Initialize LD path for library loading
    export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath packages}

    PROJECT_ROOT=$PWD
    rel_root() {
      local path
      path=$(${pkgs.coreutils}/bin/realpath --relative-to $PROJECT_ROOT $PWD)
      if [[ $path != . ]]; then echo " $path "; fi
    }

    if [[ $COLORTERM = "truecolor" ]] || [[ $COLORTERM = "24bit" ]]; then
      # We have 24-bit colors
      HEX_COLOR=${colors."24bit"}
      COLOR=$(printf "\e[38;2;%d;%d;%d2m" 0x''${HEX_COLOR:1:2} 0x''${HEX_COLOR:3:2} 0x''${HEX_COLOR:5:2})
    else
      _colors=$(tput colors)
      if [[ "$_colors" = "256" ]]; then
        # We have 8-bit colors
        COLOR=$(printf "\e[38;5;%dm" ${colors."8bit"})
      elif [[ "$_colors" = "8" ]] || [[ "$_colors" = "16" ]]; then
        # We have 3/4-bit colors
        COLOR=$(printf "\e[3%dm" ${colors."3bit"})
      fi
    fi
    PS1="\[$COLOR\][${name}]\$(rel_root)\$\[\033[0m\] "
  '';
}
