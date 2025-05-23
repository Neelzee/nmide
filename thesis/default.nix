
{ pkgs ? import <nixpkgs> {} }:
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
  envVars = {
    TEXINPUTS = ".:${rustListing}/tex/latex//:${projectRoot}/core:${projectRoot}/modules:${projectRoot}/libs:";
    TEXMFHOME = ".cache";
    TEXMFVAR = ".cache/texmf-var";
  };

  shellEnv = ''
    export PATH="${pkgs.lib.makeBinPath buildInputs}"
    export SOURCE_DATE_EPOCH="$(date +%s)"
    mkdir -p ${envVars.TEXMFVAR}
    export TEXINPUTS="${envVars.TEXINPUTS}"
    export TEXMFHOME="${envVars.TEXMFHOME}"
    export TEXMFVAR="${envVars.TEXMFVAR}"
    ln -s ${projectRoot}/core core
    ln -s ${projectRoot}/modules modules
    ln -s ${projectRoot}/libs libs
    export LANG="en_US.UTF-8";
    export LC_ALL="en_US.UTF-8";
    export LOCALE_ARCHIVE=${pkgs.glibcLocales}/lib/locale/locale-archive
  '';

  buildInputs = [
    pkgs.glibcLocales
    pkgs.coreutils
    tex
    rustListing
  ];

  thesis = pkgs.stdenvNoCC.mkDerivation {
    name = "thesis";
    src = ./.;
    inherit buildInputs;

    phases = [ "unpackPhase" "buildPhase" "installPhase" ];

    buildPhase = ''
      ${shellEnv}
      latexmk -interaction=nonstopmode \
              -latexoption=-shell-escape \
              -pdf -f "${file}".tex
    '';

    installPhase = ''
      mkdir -p $out
      cp "${file}".pdf $out/
    '';
  };
in {
  inherit thesis envVars buildInputs rustListing projectRoot shellEnv;
}
