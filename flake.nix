{
  description = "Haskell Project Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        haskellPackages = pkgs.haskellPackages;
        project = haskellPackages.callCabal2nix "nmide" ./. { };
      in
      {
        packages.default = project;

        devShells.default = pkgs.mkShell {
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
            git-cliff
            xdotool
            # TAURI END
            # HASKELL START
            zlib
            nodejs
            electron
            patchelf
            gtk2
            glib
            cairo
            pango
            atk
            gdk-pixbuf
            harfbuzz
            # HASKELL END
            haskellPackages.ghc
            haskellPackages.cabal-install
            haskellPackages.haskell-language-server
            haskellPackages.threepenny-gui
            haskellPackages.stm
          ];
        };
      }
    );
}
