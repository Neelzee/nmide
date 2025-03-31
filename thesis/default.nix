{
  pkgs ? import <nixpkgs> { },
}:

let
  tex = pkgs.texlive.combined.scheme-full;
  file = "thesis";
in
pkgs.stdenvNoCC.mkDerivation rec {
  name = "thesis";
  src = ./.;
  buildInputs = [
    pkgs.coreutils
    tex
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
}
