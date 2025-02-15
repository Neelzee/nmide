let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  name = "nmide-shell";
  # During build
  nativeBuildInputs = with pkgs; [
    pkg-config
    gobject-introspection
    cargo
    cargo-tauri
    nodejs
  ];

  # During build and runtime
  buildInputs = with pkgs; [
    # Tauri essentials
    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    pango
    webkitgtk_4_1
    openssl
    # Core TeX Live packages
    texlive.combined.scheme-full
    # Additional tools for LaTeX development
    biber # For bibliography management
    tectonic # Alternative LaTeX engine
    imagemagick # For image processing if needed
    git-cliff # For CHANGELOG
    xdotool # For keypresses
  ];
  packages = [
    (pkgs.python3.withPackages (python-pkgs: [
      python-pkgs.pygments
    ]))
  ];
}
