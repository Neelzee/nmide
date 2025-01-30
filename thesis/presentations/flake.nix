{
  description = ''
    LaTeX Beamer for Nmide slides

    Based on https://ubikium.gitlab.io/portfolio/latex-beamer-flake.html .
  '';
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
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        tex = pkgs.texlive.combined.scheme-full;
        # fix from https://github.com/NixOS/nixpkgs/issues/10008
        fontsConf = pkgs.makeFontsConf {
          fontDirectories = [
            "${tex}/share/texmf/"
          ];
        };
      in
      rec {
        packages = {
          slides =
            pkgs.runCommand "slides"
              {
                buildInputs = [
                  pkgs.coreutils
                  tex
                  pkgs.pandoc
                ];
                FONTCONFIG_FILE = fontsConf;
                src = ./.;
              }
              # Added the export line to solve this error:
              # https://discourse.nixos.org/t/fontconfig-error-no-writable-cache-directories/34447
              ''
                export XDG_CACHE_HOME="$(mktemp -d)"
                cp -r $src/* ./
                mkdir $out
                ${pkgs.pandoc}/bin/pandoc \
                  --pdf-engine=xelatex \
                  -t beamer \
                  -H header.tex default.yaml slides.md \
                  -o $out/${packages.slides.name}.pdf
              '';
        };
        defaultPackage = packages.slides;
      }
    );
}
