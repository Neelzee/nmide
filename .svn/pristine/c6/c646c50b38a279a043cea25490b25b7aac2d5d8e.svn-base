{
  description = "A LaTeX Beamer presentation built with Nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    uib-theme = {
      url = "github:martinhelso/UiB";
      # This is not a flake, just a Git repository
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      uib-theme,
    }:
    {
      packages.x86_64-linux.default =
        let
          pkgs = nixpkgs.legacyPackages.x86_64-linux;
          src = ./.;
        in
        pkgs.stdenv.mkDerivation {
          name = "beamer-slides";
          inherit src;

          buildInputs = with pkgs; [
            texlive.combined.scheme-full
            biber
            tectonic
            imagemagick
            (pkgs.python3.withPackages (python-pkgs: [
              python-pkgs.pygments
            ]))
          ];
          # Set TEXINPUTS to include the UiB theme directory
          TEXINPUTS = "${uib-theme}:";

          buildPhase = ''
            export TEXMFCACHE=$(mktemp -d)

            mkdir -p pics code figures generators
            cp -r ${../../pics}/* pics/
            cp -r ${../../code}/* code/
            cp -r ${../../figures}/* figures/
            cp -r ${./sections}/* sections/
            cp -r ${../../generators}/* generators/

            # Build the slides with latexmk
            latexmk -interaction=nonstopmode \
                    -latexoption=-shell-escape \
                    -pdf -f main.tex
          '';
          installPhase = ''
            mkdir -p $out
            cp main.pdf $out/
          '';
        };
    };
}
