{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system: {
      devShell =
        let
          pkgs = import nixpkgs {
            inherit system;
            config.allowUnsupportedSystem = true;
            overlays = [ fenix.overlays.default ];
          };
          rust-toolchain = pkgs.fenix.fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "sha256-qJPyMmsOAzf5hchL9r5V47TMHkhcnQqGFQb34lZo+C0=";
          };
          c-compiler = pkgs.pkgsCross.avr.buildPackages.gcc;
        in
          pkgs.mkShell {
            buildInputs = with pkgs; [
              rust-toolchain
              c-compiler
              avrdude
              ravedude
              gcc
            ];
            env = {
              "AVR_LIBC_PATH" = "${pkgs.pkgsCross.avr.avrlibc}/avr/lib";
            };
          };
  });
}
