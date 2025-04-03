{
  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        project = "nmide-dev";

        buildDir = "src-tauri/target/release/${project}";

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        buildInputs = with pkgs; [
          rust-bin.stable.latest.default
          gcc
          pkg-config
          rust-analyzer
          rustPlatform.rustLibSrc
          gobject-introspection
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
          git-cliff
          xdotool
        ];
      in
      {
        packages.default = pkgs.stdenv.mkDerivation {
          name = project;
          src = ./.;
          inherit buildInputs;
          buildPhase = ''
            npm i && npm run tauri build
          '';

          installPhase = ''
            mkdir -p $out
            cp -r ${buildDir} $out
          '';
        };

        defaultPackage.default = self.packages.default;

        devShells.default = pkgs.mkShell {
          inherit buildInputs;
        };
      }
    );
}
