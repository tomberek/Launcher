{
  description = "Project Description"; # TODO: Project Description

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    nix2container.url = "github:nlewo/nix2container";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs =
    inputs@{ flake-parts, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devenv.flakeModule ];
      systems = [
        "x86_64-linux"
        "i686-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      perSystem =
        {
          config,
          self',
          inputs',
          lib,
          pkgs,
          system,
          ...
        }:
        {

          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (final: prev: { cargo-tauri = final.callPackage ./nix/cargo-tauri.nix { }; }) ];
            config = { };
          };
          packages.default = pkgs.callPackage ./nix/deafult.nix { };

          devenv.shells.default = {
            name = "Project Name"; # TODO: Change Project Name
            difftastic.enable = true;
            imports = [ ];

            # https://devenv.sh/reference/options/
            packages =
              lib.optionals pkgs.stdenv.isDarwin (
                with pkgs.darwin.apple_sdk.frameworks;
                [
                  Security
                  SystemConfiguration
                  AppKit
                  WebKit
                  # Add other Darwin-specific packages here
                ]
              )
              ++ lib.optionals pkgs.stdenv.isDarwin (
                with pkgs;
                [
                  llvmPackages.libcxxStdenv
                  llvmPackages.libcxxClang
                  darwin.libobjc
                  rustup
                ]
              )
              ++ lib.optionals pkgs.stdenv.isLinux (
                with pkgs;
                [
                  glib
                  cairo
                  gdk-pixbuf
                  atk
                  pango
                  gtk3
                  openssl
                  libsoup_3
                  webkitgtk_4_1
                ]
              )
              ++ (with pkgs; [ cargo-tauri ]);

            # Define Enviroment Virables
            env = { };

            # https://devenv.sh/scripts/
            # scripts.hello.exec = "";

            # enterShell = ''

            # '';

            # https://devenv.sh/languages/
            languages.rust = {
              enable = true;
              channel = "stable";
              components = [
                "rustc"
                "cargo"
                "clippy"
                "rustfmt"
                "rust-analyzer"
              ];
            };

            languages.javascript = {
              enable = true;
              bun = {
                enable = true;
                install.enable = true;
              };
            };

            languages.typescript = {
              enable = true;
            };

            # https://devenv.sh/pre-commit-hooks/
            pre-commit.hooks = {
              nixfmt.package = pkgs.nixfmt-rfc-style;
              nixfmt.enable = true;
            };

            # https://devenv.sh/integrations/dotenv/
            dotenv.enable = true;
          };
        };
      flake = { };
    };
}
