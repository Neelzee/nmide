{
  pkgs ? import <nixpkgs> { },
}:
let
  rustListing = pkgs.stdenvNoCC.mkDerivation {
    name = "listings-rust";
    src = pkgs.fetchFromGitHub {
      owner = "denki";
      repo = "listings-rust";
      rev = "d52a3d9211ee7e065e87b0e1c15af874aefc8848";
      sha256 = "sha256-BMzjrJRJ+T73dygjhdvusciHdmXpjbAcy4ZODjMLAMY=";
    };
    installPhase = ''
      mkdir -p $out/tex/latex/listings-rust
      cp $src/listings-rust.sty $out/tex/latex/listings-rust/
    '';
  };
  tex = pkgs.texlive.combined.scheme-full;
  file = "thesis";
  projectRoot = ./..;
in
pkgs.stdenvNoCC.mkDerivation rec {
  name = "thesis";
  src = ./.;
  buildInputs = [
    pkgs.coreutils
    tex
    rustListing
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
    ln -s ${projectRoot}/core core
    export TEXINPUTS=".:${rustListing}/tex/latex//:"
    env TEXMFHOME=.cache TEXMFVAR=.cache/texmf-var \
      latexmk -interaction=nonstopmode \
              -latexoption=-shell-escape \
              -pdf -f "${file}".tex
  '';
  installPhase = ''
    mkdir -p $out
    cp "${file}".pdf $out/
  '';
}
