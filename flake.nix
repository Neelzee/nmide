{
  description = "Nmide";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    with flake-utils.lib;
    eachSystem allSystems (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        thesis = import ./thesis/default.nix { pkgs = pkgs; };
        buildInputs = with pkgs; [
          # TAURI START
          gcc
          pkg-config
          rustup
          rustPlatform.rustLibSrc
          gobject-introspection
          cargo
          cargo-tauri
          nodejs
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
          xdotool
          # TAURI END
        ];
      in
      {
        packages = {
          inherit thesis;
        };
        inherit buildInputs;
        defaultPackage = thesis;
      }
    );
}
