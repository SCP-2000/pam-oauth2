{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable-small";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; overlays = [ fenix.overlay ]; };
      in
      {
        devShell = pkgs.mkShell { buildInputs = with pkgs; [ pkgs.rust-nightly.default.toolchain pam pamtester ]; };
      }
    );
}
