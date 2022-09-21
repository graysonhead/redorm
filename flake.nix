{
    description = "redorm";

    inputs = {
        cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
        flake-utils.url = "github:numtide/flake-utils";
        rust-overlay.url = "github:oxalica/rust-overlay";
        rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
        rust-overlay.inputs.flake-utils.follows = "flake-utils";
        nixpkgs.url = "github:nixos/nixpkgs";
        flake-compat = {
            url = "github:edolstra/flake-compat";
            flake = false;
        };
    };

    outputs = inputs: with inputs;

        flake-utils.lib.eachDefaultSystem (system:
            let 
                package_name = "redorm";
                pkgs = import nixpkgs {
                    inherit system;
                    overlays = [
                            cargo2nix.overlays.default
                        ];
                };

                rustPkgs = pkgs.rustBuilder.makePackageSet {
                    rustChannel = "1.60.0";
                    packageFun = import ./Cargo.nix;
                };

                workspaceShell = rustPkgs.workspaceShell {};

                ci = pkgs.rustBuilder.runTests rustPkgs.workspace.cargo2nix {};

            in rec {
                packages = {
                    redorm = (rustPkgs.workspace.redorm {}).bin;
                    default = packages.redorm;
                };
                devShell = workspaceShell;
            }
        );
}