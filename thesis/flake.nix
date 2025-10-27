{
  description = "Nmide Thesis";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachSystem flake-utils.lib.allSystems (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (import ./default.nix { inherit pkgs; }) thesis envVars buildInputs shellEnv;
      in
      {
        packages.default = thesis;

        devShells.default = pkgs.mkShell {
          inherit buildInputs;

          TEXINPUTS = envVars.TEXINPUTS;

          shellHook = ''
            ${shellEnv}
          '';
        };
      });
}

