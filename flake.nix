{
  description = "Palm Nix";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
  };

  outputs =
    {
      nixpkgs,
      fenix,
      ...
    }:
    let
      version = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages."${version}";
    in
    {
      devShells."${version}".default = pkgs.mkShell {
        buildInputs = [
          (
            with fenix.packages."${version}";
            with stable;
            combine [
              rustc
              cargo
              llvm-tools-preview
              targets.x86_64-unknown-linux-gnu.stable.rust-std
              rust-analyzer
              clippy
            ]
          )
          pkgs.postgresql
        ];
        CARGO_HOME = "../.cargo/";
      };
    };
}
