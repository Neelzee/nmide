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
    flake-utils.lib.eachSystem flake-utils.lib.allSystems (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        thesis = import ./default.nix { pkgs = pkgs; };
      in
      {
        packages = {
          inherit thesis;
        };
        defaultPackage = thesis;
      }
    );
}
