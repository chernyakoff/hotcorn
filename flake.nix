{
  description = "Hotcorn - Hot Corners and Edge Triggers for Hyprland";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
      in {
        # Build Rust package
        packages.hotcorn = pkgs.rustPlatform.buildRustPackage {
          pname = "hotcorn";
          version = "0.1.0";
          src = ./.;
          cargoLock = ./Cargo.lock;
          buildInputs = [ pkgs.pkg-config pkgs.libxcb ];
        };

        # Dev shell for development
        devShells.default = pkgs.mkShell {
          buildInputs = [ pkgs.rustc pkgs.cargo pkgs.pkg-config pkgs.libxcb ];
          shellHook = ''
            echo "Hotcorn dev shell ready!"
          '';
        };
      });
}
