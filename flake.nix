{
  description = "Rust development environment with stable toolchain via rustup";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            clang
            llvmPackages.libclang
            rustup
            cargo-expand
            pkg-config
            openssl
          ];
        };
      });
}
