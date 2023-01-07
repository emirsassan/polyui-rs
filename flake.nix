{
  description = "PolyUI workspace flake for the Rust monorepo.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{self, ...}:
    inputs.utils.lib.eachDefaultSystem (system: let
      pkgs = import inputs.nixpkgs { inherit system; };
      fenix = inputs.fenix.packages.${system};
      utils = inputs.utils.lib;

      toolchain = with fenix;
        combine [
          minimal.rustc minimal.cargo
        ];
      
      naersk = inputs.naersk.lib.${system}.override {
        rustc = toolchain;
        cargo = toolchain;
      };
      
      deps = with pkgs; {
        global = [
          openssl pkg-config gcc
        ];
        shell = [
          (with fenix; combine [toolchain default.clippy complete.rust-src rust-analyzer])
          git
          jdk17 jdk8
          nodejs-19_x
        ];
      };
    in {
      packages = {
        core = naersk.buildPackage {
          pname = "polyui_core";
          src = ./core;
          buildInputs = deps.global;
          cargoBuildOptions = x: x ++ ["-p" "polyui_core"];
        };
      };

      apps = {
        core = utils.mkApp {
          drv = self.packages.${system}.core;
        };
        core-dev = utils.mkApp {
          drv = self.packages.${system}.core.overrideAttrs (old: old // {
            release = false;
          });
        };
      };

      devShell = pkgs.mkShell {
        buildInputs = with deps;
          global ++ shell;
      };
    });
}