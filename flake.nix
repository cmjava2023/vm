{
  inputs.flake-compat = {
    url = "github:edolstra/flake-compat";
    flake = false;
  };

  inputs.nixpkgs.url = "github:NixOs/nixpkgs/nixos-unstable";

  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  inputs.nix-filter = {
    url = "github:numtide/nix-filter";
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    nix-filter,
    ...
  } @ inputs: let
    packageName = "cmjava";

    forSystems = function:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
      ] (system: let
        pkgs = import nixpkgs {
          inherit system;

          overlays = [
            (final: prev: {
              ${packageName} = self.packages.${system}.${packageName};
            })
          ];
        };

        fenix-pkgs = fenix.packages.${system};
        fenix-channel = fenix-pkgs.toolchainOf {
          channel = "nightly";
          date = builtins.replaceStrings ["nightly-"] [""] (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml)).toolchain.channel;
          sha256 = "sha256-SzEeSoO54GiBQ2kfANPhWrt0EDRxqEvhIbTt2uJt/TQ=";
        };
      in
        function {inherit system pkgs fenix-pkgs fenix-channel;});
  in {
    formatter = forSystems ({pkgs, ...}: pkgs.alejandra);

    packages = forSystems ({
      pkgs,
      fenix-channel,
      system,
      ...
    }: {
      ${packageName} = pkgs.callPackage (./. + "/nix/packages/${packageName}.nix") {
        inherit packageName;
        flake-self = self;
        nix-filter = import inputs.nix-filter;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = fenix-channel.toolchain;
          rustc = fenix-channel.toolchain;
        };
      };
      default = self.packages.${system}.${packageName};
    });

    devShells = forSystems ({
      pkgs,
      fenix-pkgs,
      fenix-channel,
      ...
    }: let
      fenixRustToolchain = fenix-channel.withComponents [
        "cargo"
        "clippy-preview"
        "rust-src"
        "rustc"
        "rustfmt-preview"
      ];
    in {
      default = pkgs.callPackage (./. + "/nix/dev-shells/${packageName}.nix") {
        inherit fenixRustToolchain packageName;
      };
      ci = pkgs.callPackage ./nix/dev-shells/ci.nix {
        inherit fenixRustToolchain packageName;
      };
    });
  };
}
