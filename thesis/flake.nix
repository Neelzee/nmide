{
  description = "Nmide Thesis";
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
        tex = pkgs.texlive.combined.scheme-full;
        file = "report";
      in
      rec {
        packages = {
          document = pkgs.stdenvNoCC.mkDerivation rec {
            name = "latex-demo-document";
            src = self;
            buildInputs = [
              pkgs.coreutils
              tex
              pkgs.biber
              (pkgs.python3.withPackages (python-pkgs: [
                python-pkgs.pygments
              ]))
            ];
            phases = [
              "unpackPhase"
              "buildPhase"
              "installPhase"
            ];
            buildPhase = ''
              export PATH="${pkgs.lib.makeBinPath buildInputs}"
              export SOURCE_DATE_EPOCH="$(date +%s)"
              mkdir -p .cache/texmf-var
              env TEXMFHOME=.cache TEXMFVAR=.cache/texmf-var \
              latexmk -interaction=nonstopmode \
                    -latexoption=-shell-escape \
                    -pdf -f "${file}".tex
            '';
            installPhase = ''
              mkdir -p $out
              cp "${file}".pdf $out/
            '';
          };
        };
        defaultPackage = packages.document;
      }
    );
}
