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
        project = haskellPackages.callCabal2nix "haskell-electron" ./. { };
      in
      {
        packages.default = project;

        devShells.default = pkgs.mkShell {
          buildInputs = [
            haskellPackages.ghc
            haskellPackages.cabal-install
            haskellPackages.haskell-language-server
            haskellPackages.threepenny-gui
            pkgs.zlib
            pkgs.nodejs
            pkgs.electron
            pkgs.patchelf
            pkgs.gtk2
            pkgs.glib
            pkgs.cairo
            pkgs.pango
            pkgs.atk
            pkgs.gdk-pixbuf
            pkgs.harfbuzz
          ];

        };
      }
    );
}
