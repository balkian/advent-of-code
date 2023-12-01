{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    # rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, nixpkgs, flake-utils,fenix }:
    flake-utils.lib.eachDefaultSystem (system: {
      packages.default =
        let
          toolchain = fenix.packages.${system}.minimal.toolchain;
          pkgs = nixpkgs.legacyPackages.${system};
        in

        (pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        }).buildRustPackage {
          pname = "example";
          version = "0.1.0";

          src = ./2023/rust/;

          cargoLock.lockFile = ./2023/rust/Cargo.lock;
        };
    });
    #   let
    #   pkgs = nixpkgs.legacyPackages.x86_64-linux.pkgs;
    #   fooScript = pkgs.writeScriptBin "foo.sh" ''
    #     #!/bin/sh
    #     echo $FOO
    #   '';
    # in {
    #   devShells.x86_64-linux.default = pkgs.mkShell {
    #     name = "AoC build environment";
    #     buildInputs = [
    #       pkgs.python39
    #       pkgs.python39Packages.tox
    #       pkgs.python39Packages.flake8
    #       pkgs.python39Packages.requests
    #       pkgs.python39Packages.ipython
    #       fooScript
    #     ];
    #     shellHook = ''
    #       echo "Welcome in $name"
    #       export FOO="BAR"
    #     '';
    #   };
    # };
    # flake-utils.lib.eachDefaultSystem
    #   (system:
    #     let
    #       overlays = [ (import fenix) ];
    #       pkgs = import nixpkgs {
    #         inherit system overlays;
    #       };
    #     in
    #     with pkgs;
    #     {
    #       devShells.default = mkShell {
    #         buildInputs = [ rust-bin.stable.latest.default ];
    #       };
    #     }
    #   );
}
