{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        # flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, crane, nixpkgs, flake-utils,fenix }:
    flake-utils.lib.eachDefaultSystem (system: 
      let
        craneLib = crane.lib.${system}.overrideToolchain
          fenix.packages.${system}.minimal.toolchain;
        pkgs = nixpkgs.legacyPackages.${system};
        my-crate = craneLib.buildPackage {
            src = ./. ;
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl ];
            doCheck = false;
          };


      in
      {
      checks = { inherit my-crate; };
      packages.default = my-crate;
      devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Extra inputs can be added here; cargo and rustc are provided by default
          # from the toolchain that was specified earlier.
          packages = [
            # rustWithWasiTarget
          ];
        };
    });
}
