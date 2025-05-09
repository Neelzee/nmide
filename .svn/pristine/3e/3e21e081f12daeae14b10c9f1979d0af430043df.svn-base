{
  description = "Nmide";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachSystem flake-utils.lib.allSystems (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        thesisPkg = import ./thesis/default.nix { inherit pkgs; };
        thesis = thesisPkg.thesis;
        envVars = thesisPkg.envVars or {};
        texBuildInputs = thesisPkg.buildInputs or [];
        tauriBuildInputs = with pkgs; [
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
        ];

        buildInputs = texBuildInputs ++ tauriBuildInputs;

      in {
        packages = {
          thesis = thesis;
        };

        defaultPackage = thesis;

        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          TEXINPUTS = envVars.TEXINPUTS or "";

          shellHook = ''
            export LANG=en_US.UTF-8
            export LC_ALL=en_US.UTF-8
            export LANGUAGE=en_US
            export SOURCE_DATE_EPOCH="$(date +%s)"
            export TEXMFHOME=${envVars.TEXMFHOME}
            export TEXMFVAR=${envVars.TEXMFVAR}
            mkdir -p ${envVars.TEXMFVAR}
          '';
        };
      });
}
