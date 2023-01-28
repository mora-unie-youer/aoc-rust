{ pkgs, ... }:

let
  # Until I have nice devshell flake
  name = "aoc-rust";
  # 0-255 or hex color
  color = "#17BEBB";
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

    # get_colors (int?) - number of bit colors terminal supports
    # - <empty> -> terminal doesn't support colors
    # - 4 -> 4-bit
    # - 8 -> 8-bit
    # - 24 -> 24-bit
    get_colors() {
      if [[ $COLORTERM = "truecolor" ]] || [[ $COLORTERM = "24bit" ]]; then
        echo 24
      else
        local colors=$(tput colors)
        if [[ "$colors" = "256" ]]; then
          echo 8
        elif [[ "$colors" = "8" ]] || [[ "$colors" = "16" ]]; then
          echo 4
        fi
      fi
    }
    COLORS=$(get_colors)
    USER_COLOR=${color}

    if [[ "$USER_COLOR" =~ ^\# ]]; then
      # Parse HEX color
      if [[ "$COLORS" = "24" ]]; then
        COLOR=$(printf "\e[38;2;%d;%d;%d2m" 0x''${USER_COLOR:1:2} 0x''${USER_COLOR:3:2} 0x''${USER_COLOR:5:2})
      else
        # Fallback color on not having 24bit coloring
        COLOR="\e[38;5;202m"
      fi
    else
      # Generate color string
      COLOR="\e[38;5;''${USER_COLOR}m"
    fi
    if [[ "$COLORS" -lt "8" ]]; then
      # Fallback color on not having 24bit/8bit coloring
      COLOR="\e[31m"
    fi

    PS1="\[$COLOR\][${name}]\$(rel_root)\$\[\033[0m\] "
  '';
}
